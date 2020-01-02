extern crate async_std;
extern crate async_trait;
extern crate futures;
extern crate lz4_compress;
extern crate snap;

pub mod authenticators;
pub mod query;

mod compressor;
mod get_compressor_trait;
mod get_transport_trait;
mod transport;
mod transport_builder_trait;
mod transport_tcp;

pub use compressor::Compression;
pub use get_compressor_trait::GetCompressor;
pub use get_transport_trait::GetTransport;
pub use transport::CDRSTransport;
pub use transport_builder_trait::CDRSTransportBuilder;
pub use transport_tcp::{TcpTransportBuilder, TransportTcp};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
