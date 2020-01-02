// use std::net;
use std::io::{IoSlice, IoSliceMut};
use std::marker::Unpin;
use std::task::Poll;
use std::time;

use async_std::io;
use async_std::io::{Read, Write};
use async_std::net;
use async_std::pin::Pin;
use async_std::prelude::*;
use async_std::task::Context;
use futures::io::AsyncRead;

use super::transport::CDRSTransport;
use super::transport_builder_trait::CDRSTransportBuilder;

/// Default Tcp transport.
pub struct TransportTcp {
  tcp: net::TcpStream,
  addr: String,
}

impl TransportTcp {
  /// Constructs a new `TransportTcp`.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use cdrs::transport::TransportTcp;
  /// let addr = "127.0.0.1:9042";
  /// let tcp_transport = TransportTcp::new(addr).unwrap();
  /// ```
  pub async fn new(addr: &str) -> io::Result<TransportTcp> {
    net::TcpStream::connect(addr)
      .await
      .map(|socket| TransportTcp {
        tcp: socket,
        addr: addr.to_string(),
      })
  }
}

impl Unpin for TransportTcp {}

impl Read for TransportTcp {
  fn poll_read(self: Pin<&mut Self>, ctx: &mut Context, buf: &mut [u8]) -> Poll<io::Result<usize>> {
    unimplemented!()
  }

  fn poll_read_vectored(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    bufs: &mut [IoSliceMut<'_>],
  ) -> Poll<io::Result<usize>> {
    unimplemented!()
  }
}

impl Write for TransportTcp {
  fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
    unimplemented!()
  }

  fn poll_write_vectored(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    bufs: &[IoSlice<'_>],
  ) -> Poll<io::Result<usize>> {
    unimplemented!()
  }

  fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    unimplemented!()
  }

  fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    unimplemented!()
  }
}

impl CDRSTransport for TransportTcp {
  fn try_clone(&self) -> io::Result<TransportTcp> {
    unimplemented!()
    // net::TcpStream::connect(self.addr.as_str()).map(|socket| TransportTcp {
    //   tcp: socket,
    //   addr: self.addr.clone(),
    // })
  }

  fn close(&mut self, close: net::Shutdown) -> io::Result<()> {
    self.tcp.shutdown(close)
  }

  fn set_timeout(&mut self, dur: Option<time::Duration>) -> io::Result<()> {
    unimplemented!()
    // self
    //   .tcp
    //   .set_read_timeout(dur)
    //   .and_then(|_| self.tcp.set_write_timeout(dur))
  }

  fn is_alive(&self) -> bool {
    self.tcp.peer_addr().is_ok()
  }
}

pub struct TcpTransportBuilder {
  addr: String,
}

impl TcpTransportBuilder {
  pub fn new(addr: String) -> Self {
    TcpTransportBuilder { addr }
  }
}

// impl CDRSTransportBuilder<TransportTcp> for TcpTransportBuilder {
//   fn create(&self) -> io::Result<TransportTcp> {
//     TransportTcp::new(self.addr.as_str())
//   }
// }
