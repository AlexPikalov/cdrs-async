extern crate async_trait;
extern crate cassandra_proto;
extern crate futures;
extern crate log;
extern crate lz4_compress;
extern crate snap;
extern crate transport as cdrs_transport;
#[cfg(feature = "async_std")]
extern crate transport_async;
#[cfg(feature = "tok_io")]
extern crate transport_tokio;

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
pub mod transport {
    pub use cdrs_transport::CDRSTransport;
    #[cfg(feature = "tok_io")]
    pub mod tokio {
        pub use transport_tokio::TransportTcp;
        pub use transport_tokio::TransportTls;
    }

    #[cfg(feature = "async_std")]
    pub mod async_std {
        pub use transport_async::TransportTcp;
        pub use transport_async::TransportTls;
    }
}
