use std::{
  io::{IoSlice, IoSliceMut},
  marker::Unpin,
  pin::Pin,
  task::{Context, Poll},
};

use async_std::{
  io,
  io::{Read, Write},
  net,
};
use async_trait::async_trait;

use super::transport::CDRSTransport;

/// Default Tcp transport.
pub struct TransportTcp {
  tcp: net::TcpStream,
  _addr: String,
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
        _addr: addr.to_string(),
      })
  }
}

impl Unpin for TransportTcp {}

impl Read for TransportTcp {
  fn poll_read(
    mut self: Pin<&mut Self>,
    cx: &mut Context,
    buf: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.tcp).poll_read(cx, buf)
  }

  fn poll_read_vectored(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    bufs: &mut [IoSliceMut<'_>],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.tcp).poll_read_vectored(cx, bufs)
  }
}

impl Write for TransportTcp {
  fn poll_write(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.tcp).poll_write(cx, buf)
  }

  fn poll_write_vectored(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    bufs: &[IoSlice<'_>],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.tcp).poll_write_vectored(cx, bufs)
  }

  fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.tcp).poll_flush(cx)
  }

  fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.tcp).poll_close(cx)
  }
}

#[async_trait]
impl CDRSTransport for TransportTcp {
  // FIXME:
  // async fn try_clone(&self) -> io::Result<TransportTcp> {
  //   net::TcpStream::connect(self.addr.as_str())
  //     .await
  //     .map(|socket| TransportTcp {
  //       tcp: socket,
  //       addr: self.addr.clone(),
  //     })
  // }

  fn close(&mut self, close: net::Shutdown) -> io::Result<()> {
    self.tcp.shutdown(close)
  }

  fn is_alive(&self) -> bool {
    self.tcp.peer_addr().is_ok()
  }
}
