extern crate async_trait;
extern crate cassandra_proto;
extern crate futures;
extern crate log;
extern crate lz4_compress;
extern crate snap;
extern crate transport;
#[cfg(feature = "async_std")]
extern crate transport_async;

pub mod authenticators;
pub mod query;

pub(crate) mod frame_channel;

mod compressor;
mod pager;
mod session;
mod utils;

pub use cassandra_proto::compression::Compressor;
pub use compressor::Compression;
pub use pager::PageSize;
pub use session::Session;
pub use transport::CDRSTransport;
pub use transport_async::transport_tcp::TransportTcp;
pub use transport_async::transport_tls::TransportTls;
