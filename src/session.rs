use std::{
  collections::HashMap,
  io,
  pin::Pin,
  task::{Context, Poll},
};

use async_std::{future::poll_fn, prelude::*};
use async_tls::TlsConnector;
use cassandra_proto::{
  error,
  frame::{parser_async::convert_frame_into_result, Frame, IntoBytes, Opcode},
  query::{Query, QueryBatch, QueryParams},
};
use futures::stream::Stream;

use crate::{
  async_trait::async_trait,
  authenticators::Authenticator,
  compressor::Compression,
  frame_channel::FrameChannel,
  pager::{PageSize, SessionPager},
  query::{BatchExecutor, ExecExecutor, PrepareExecutor, PreparedQuery, QueryExecutor},
  transport::CDRSTransport,
  utils::prepare_flags,
  TransportTcp, TransportTls,
};

type StreamId = u16;

/// Session structure which allows clients making requests to a server.
pub struct Session<T> {
  channel: FrameChannel<T>,
  responses: HashMap<StreamId, Frame>,
  authenticator: Authenticator,
}

macro_rules! receive_frame {
  ($this: expr, $stream_id: expr) => {
    poll_fn(|cx: &mut Context| {
      if let Some(response) = $this.responses.remove(&$stream_id) {
        return Poll::Ready(convert_frame_into_result(response));
      }

      match Pin::new(&mut $this.channel).poll_next(cx) {
        Poll::Ready(Some(frame)) => {
          if frame.stream == $stream_id {
            return Poll::Ready(convert_frame_into_result(frame));
          } else {
            $this.responses.insert(frame.stream, frame);
            return Poll::Pending;
          }
        }
        Poll::Ready(None) => Poll::Ready(Err("stream was terminated".into())),
        Poll::Pending => Poll::Pending,
      }
    })
  };
}

impl Session<TransportTcp> {
  pub async fn connect<Addr: ToString>(
    addr: Addr,
    compressor: Compression,
    authenticator: Authenticator,
  ) -> error::Result<Self> {
    let transport = TransportTcp::new(&addr.to_string()).await?;
    let channel = FrameChannel::new(transport, compressor);
    let responses = HashMap::new();

    let mut session = Session {
      channel,
      responses,
      authenticator,
    };

    session.startup().await?;

    Ok(session)
  }

  /// Converts `Session` into `SessionPager`
  pub fn into_pager(self, page_size: PageSize) -> SessionPager<TransportTcp> {
    SessionPager::new(self, page_size)
  }
}

impl Session<TransportTls> {
  pub async fn connect_tls<Addr: ToString>(
    (addr, connector): (Addr, TlsConnector),
    compressor: Compression,
    authenticator: Authenticator,
  ) -> error::Result<Self> {
    let transport = TransportTls::new(&addr.to_string(), connector).await?;
    let channel = FrameChannel::new(transport, compressor);
    let responses = HashMap::new();

    let mut session = Session {
      channel,
      responses,
      authenticator,
    };

    session.startup().await?;

    Ok(session)
  }
}

impl<T: CDRSTransport> Session<T> {
  async fn startup(&mut self) -> error::Result<()> {
    let ref mut compression = Compression::None;
    let startup_frame = Frame::new_req_startup(compression.as_str());
    let stream = startup_frame.stream;

    self.channel.write(&startup_frame.into_cbytes()).await?;
    let start_response = receive_frame!(self, stream).await?;

    if start_response.opcode == Opcode::Ready {
      return Ok(());
    }

    if start_response.opcode == Opcode::Authenticate {
      let body = start_response.get_body()?;
      let authenticator = body.get_authenticator().expect(
        "Cassandra Server did communicate that it neededs
                authentication but the auth schema was missing in the body response",
      );

      // This creates a new scope; avoiding a clone
      // and we check whether
      // 1. any authenticators has been passed in by client and if not send error back
      // 2. authenticator is provided by the client and `auth_scheme` presented by
      //      the server and client are same if not send error back
      // 3. if it falls through it means the preliminary conditions are true

      let auth_check = self
        .authenticator
        .get_cassandra_name()
        .ok_or(error::Error::General(
          "No authenticator was provided".to_string(),
        ))
        .map(|auth| {
          if authenticator != auth {
            let io_err = io::Error::new(
              io::ErrorKind::NotFound,
              format!(
                "Unsupported type of authenticator. {:?} got,
                             but {} is supported.",
                authenticator, auth
              ),
            );
            return Err(error::Error::Io(io_err));
          }
          Ok(())
        });

      if let Err(err) = auth_check {
        return Err(err);
      }

      let auth_token_bytes =
        self
          .authenticator
          .get_auth_token()
          .into_plain()
          .ok_or(error::Error::from(
            "Authentication error: cannot get auth token",
          ))?;
      let auth_response = Frame::new_req_auth_response(auth_token_bytes);
      let response_stream = auth_response.stream;

      self.channel.write(&auth_response.into_cbytes()).await?;
      receive_frame!(self, response_stream).await?;

      return Ok(());
    }

    unreachable!();
  }
}

#[async_trait]
impl<T: CDRSTransport> QueryExecutor for Session<T> {
  async fn query_with_params_tw<Q: ToString + Send>(
    mut self: Pin<&mut Self>,
    query: Q,
    query_params: QueryParams,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let query = Query {
      query: query.to_string(),
      params: query_params,
    };

    let flags = prepare_flags(with_tracing, with_warnings);
    let query_frame = Frame::new_query(query, flags);
    let stream = query_frame.stream;

    // send frame
    self.channel.write(&query_frame.into_cbytes()).await?;
    receive_frame!(self, stream).await
  }
}

#[async_trait]
impl<T: CDRSTransport> PrepareExecutor for Session<T> {
  async fn prepare_tw<Q: ToString + Send>(
    mut self: Pin<&mut Self>,
    query: Q,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<PreparedQuery> {
    let flags = prepare_flags(with_tracing, with_warnings);

    let query_frame = Frame::new_req_prepare(query.to_string(), flags);
    let stream = query_frame.stream;

    self.channel.write(&query_frame.into_cbytes()).await?;

    let prepared_id = receive_frame!(self, stream)
      .await?
      .get_body()?
      .into_prepared()
      .ok_or(error::Error::from(
        "Cannot get prepared query ID from a response",
      ))?
      .id;

    Ok(prepared_id)
  }
}

#[async_trait]
impl<T: CDRSTransport> ExecExecutor for Session<T> {
  async fn exec_with_params_tw(
    mut self: Pin<&mut Self>,
    prepared: &PreparedQuery,
    query_parameters: QueryParams,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let flags = prepare_flags(with_tracing, with_warnings);
    let executor_frame = Frame::new_req_execute(prepared, query_parameters, flags);
    let stream = executor_frame.stream;

    self.channel.write(&executor_frame.into_cbytes()).await?;
    receive_frame!(self, stream).await
  }
}

#[async_trait]
impl<T: CDRSTransport> BatchExecutor for Session<T> {
  async fn batch_with_params_tw(
    mut self: Pin<&mut Self>,
    batch: QueryBatch,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let flags = prepare_flags(with_tracing, with_warnings);
    let batch_frame = Frame::new_req_batch(batch, flags);
    let stream = batch_frame.stream;

    self.channel.write(&batch_frame.into_cbytes()).await?;
    receive_frame!(self, stream).await
  }
}
