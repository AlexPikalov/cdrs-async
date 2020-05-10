use transport::CDRSTransport;

use async_trait::async_trait;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net;

pub struct TransportTcp {
    tcp: net::TcpStream,
}

impl TransportTcp {
    pub async fn new(addr: &str) -> io::Result<TransportTcp> {
        net::TcpStream::connect(addr)
            .await
            .map(|stream| TransportTcp { tcp: stream })
    }
}

impl futures::AsyncRead for TransportTcp {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.tcp).poll_read(cx, buf)
    }
}

impl futures::AsyncWrite for TransportTcp {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.tcp).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.tcp).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.tcp).poll_shutdown(cx)
    }
}

#[async_trait]
impl CDRSTransport for TransportTcp {
    fn close(&mut self, close: std::net::Shutdown) -> io::Result<()> {
        self.tcp.shutdown(close)
    }

    fn is_alive(&self) -> bool {
        self.tcp.peer_addr().is_ok()
    }
}
