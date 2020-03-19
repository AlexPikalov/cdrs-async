use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use cassandra_proto::{error::Result as CDRSResult, frame::Frame};
use futures::stream::Stream;

use crate::frame_channel::FrameChannel;
use crate::transport::CDRSTransport;

pub struct ServerResp<T> {
  frame_channel: Arc<Mutex<FrameChannel<T>>>,
}

impl<T: CDRSTransport> Future for ServerResp<T> {
  type Output = CDRSResult<Frame>;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    match self.frame_channel.clone().lock() {
      Ok(channel) => Pin::new(channel).poll_next(),
      Err(_) => {}
    }
    return Poll::Pending;
  }
}
