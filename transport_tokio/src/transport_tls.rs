use transport::CDRSTransport;

use async_trait::async_trait;
use native_tls::TlsConnector as NativeTlsConnector;
use std::io;
use std::io::ErrorKind;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net;
use tokio_native_tls::{TlsConnector, TlsStream};

pub struct TransportTls {
    tls: TlsStream<net::TcpStream>,
}

impl TransportTls {
    pub async fn new(addr: &str) -> io::Result<TransportTls> {
        let socket = net::TcpStream::connect(&addr).await?;
        let native_tls = NativeTlsConnector::builder().build().map_err(|_| {
            io::Error::new(ErrorKind::Other, "Could not build native tls connector")
        })?;
        TlsConnector::from(native_tls)
            .connect(addr, socket)
            .await
            .map(|stream| TransportTls { tls: stream })
            .map_err(|e| io::Error::new(ErrorKind::ConnectionRefused, e))
    }
}

impl futures::AsyncRead for TransportTls {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.tls).poll_read(cx, buf)
    }
}

impl futures::AsyncWrite for TransportTls {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.tls).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.tls).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.tls).poll_shutdown(cx)
    }
}

#[async_trait]
impl CDRSTransport for TransportTls {
    fn close(&mut self, _: std::net::Shutdown) -> io::Result<()> {
        self.tls.get_mut().shutdown()
    }

    fn is_alive(&self) -> bool {
        self.tls.get_ref().get_ref().get_ref().peer_addr().is_ok()
    }
}
