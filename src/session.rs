use std::collections::HashMap;

use async_std::future::poll_fn;
use async_std::pin::Pin;
use async_std::prelude::*;
use async_std::task::{Context, Poll};

use futures::stream::Stream;

use cassandra_proto::{
  error,
  frame::{parser_async::convert_frame_into_result, Frame, IntoBytes},
  query::{Query, QueryParams},
};

use crate::{
  async_trait::async_trait, compressor::Compression, frame_channel::FrameChannel,
  query::QueryExecutor, transport_tcp::TransportTcp, utils::prepare_flags,
};

type StreamId = u16;

pub struct Session {
  channel: FrameChannel<TransportTcp>,
  compressor: Compression,
  responses: HashMap<StreamId, Frame>,
}

impl From<TransportTcp> for Session {
  fn from(transport: TransportTcp) -> Session {
    Session {
      channel: FrameChannel::new(transport, Compression::None),
      compressor: Compression::None,
      responses: HashMap::new(),
    }
  }
}

#[async_trait]
impl QueryExecutor for Session {
  fn get_compression(self: Pin<&mut Self>) -> Compression {
    self.compressor
  }

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

    println!("stream id {:?}", stream);

    // TODO: return a Future that will
    // * inspect self.responses on poll and try to find a response with a given stream id
    // * if not found call.channel.next() and check stream id of a given frame
    // * if stream didn't match save into self.responses

    poll_fn(|cx: &mut Context| {
      println!("polling response");
      if let Some(response) = self.responses.remove(&stream) {
        return Poll::Ready(convert_frame_into_result(response));
      }

      match Pin::new(&mut self.channel).poll_next(cx) {
        Poll::Ready(Some(frame)) => {
          if frame.stream == stream {
            return Poll::Ready(convert_frame_into_result(frame));
          } else {
            self.responses.insert(frame.stream, frame);
            return Poll::Pending;
          }
        }
        Poll::Ready(None) => Poll::Ready(Err("stream was terminated".into())),
        Poll::Pending => Poll::Pending,
      }
    })
    .await

    // if let Some(frame) = self.channel.next().await {
    //   let received_stream_id = frame.stream;
    //   todo!();
    //   // self.responses.insert(stream_id, frame);
    // }

    // receive all available frames and find the one that matches the stream

    // loop {
    // parse_frame_async(&mut self.transport, &compressor);
    // match parse_frame_async(&mut self.transport, &compressor) {
    //   // Poll::Ready(_) => {}
    // }
    // }

    // todo!()
  }
}
