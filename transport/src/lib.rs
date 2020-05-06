use {
    async_trait::async_trait,
    std::{io, marker::Unpin, net},
};

use futures::{AsyncRead, AsyncWrite};

/// Generic transport trait which is implemented by transports provided by CDRS.
///
/// It requires that implementor had following traits implementations: `Sized`,
/// `Send`, `Unpin`, async `Read` and `Write`.
#[async_trait]
pub trait CDRSTransport: Sized + AsyncRead + AsyncWrite + Send + Sync + Unpin {
    // TODO: uncomment it
    // /// Creates a new independently owned handle to the underlying socket.
    // ///
    // /// The returned TcpStream is a reference to the same stream that this object references.
    // /// Both handles will read and write the same stream of data, and options set on one stream
    // /// will be propagated to the other stream.
    // async fn try_clone(&self) -> io::Result<Self>;

    /// Shuts down the read, write, or both halves of this connection.
    fn close(&mut self, close: net::Shutdown) -> io::Result<()>;

    /// Method that checks that transport is alive
    fn is_alive(&self) -> bool;
}
