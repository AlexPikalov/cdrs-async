use std::io;
use std::pin::Pin;

use async_std::{
  io::{IoSlice, Write},
  prelude::*,
};
use cassandra_proto::frame::{parser_async::parse_frame_async, Frame, IntoBytes};
use futures::{
  sink::Sink,
  stream::Stream,
  task::{Context, Poll},
};
use log::error;

use crate::{compressor::Compression, transport::CDRSTransport};

const READING_BUFFER_SIZE: usize = 1_000;

pub struct FrameChannel<T> {
  transport: T,
  sending_buffer: Vec<u8>,
  receving_buffer: Vec<u8>,
  compressor: Compression,
  is_terminated: bool,
}

impl<T> FrameChannel<T> {
  pub fn new(transport: T, compressor: Compression) -> FrameChannel<T> {
    FrameChannel {
      transport,
      sending_buffer: Vec::with_capacity(8_000),
      receving_buffer: Vec::with_capacity(8_000),
      compressor,
      is_terminated: false,
    }
  }
}

impl<T: CDRSTransport> Sink<Frame> for FrameChannel<T> {
  type Error = io::Error;

  fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
    cx.waker().wake_by_ref();

    Poll::Ready(Ok(()))
  }

  fn start_send(self: Pin<&mut Self>, item: Frame) -> Result<(), Self::Error> {
    self
      .get_mut()
      .sending_buffer
      .extend_from_slice(&item.into_cbytes());

    Ok(())
  }

  fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
    let buff = self.sending_buffer.split_off(0);
    let mut transport = Pin::new(&mut self.transport);

    transport.write(&buff);
    println!("before poll flushing");
    let p = transport.poll_flush(cx);

    p
  }

  fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
    Pin::new(&mut self.transport).poll_close(cx)
  }
}

impl<T: CDRSTransport> Write for FrameChannel<T> {
  fn poll_write(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.transport).poll_write(cx, buf)
  }

  fn poll_write_vectored(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    bufs: &[IoSlice<'_>],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.transport).poll_write_vectored(cx, bufs)
  }

  fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.transport).poll_flush(cx)
  }

  fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.transport).poll_close(cx)
  }
}

impl<T: CDRSTransport> Stream for FrameChannel<T> {
  type Item = Frame;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
    let compressor = self.compressor;
    let transport = Pin::new(&mut self.transport);
    let mut buffer_slice = [0u8; READING_BUFFER_SIZE];

    match transport.poll_read(cx, &mut buffer_slice) {
      Poll::Ready(result) => match result {
        Ok(n) => {
          self.receving_buffer.extend_from_slice(&buffer_slice[0..n]);
          if n == READING_BUFFER_SIZE || n == 0 {
            return Poll::Pending;
          } else {
            // n < READING_BUFFER_SIZE means the function can proceed further
          }
        }
        Err(err) => {
          error!("CDRS frame_channel: {:?}", err);
          self.is_terminated = true;
          return Poll::Ready(None);
        }
      },
      Poll::Pending => {
        return Poll::Pending;
      }
    }

    let mut buffer_cursor = io::Cursor::new(&mut self.receving_buffer);

    match parse_frame_async(&mut buffer_cursor, &compressor) {
      Err(err) => {
        error!("CDRS frame_channel: parse frame error {:?}", err);
        self.is_terminated = true;
        return Poll::Ready(None);
      }
      Ok(Some(frame)) => {
        let cursor_position = buffer_cursor.position();
        self.receving_buffer = buffer_cursor
          .into_inner()
          .split_off(cursor_position as usize);

        return Poll::Ready(Some(frame));
      }
      Ok(None) => {
        return Poll::Pending;
      }
    }
  }
}
