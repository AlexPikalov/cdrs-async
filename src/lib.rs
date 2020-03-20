extern crate async_std;
extern crate async_trait;
extern crate cassandra_proto;
extern crate futures;
extern crate log;
extern crate lz4_compress;
extern crate snap;

pub mod authenticators;
pub mod query;

pub(crate) mod frame_channel;

mod compressor;
mod get_compressor_trait;
mod get_transport_trait;
// mod server_resp;
mod session;
mod transport;
mod transport_builder_trait;
mod transport_tcp;
mod utils;

pub use compressor::Compression;
// pub(crate) use get_compressor_trait::GetCompressor;
// pub(crate) use get_transport_trait::GetTransport;
// pub use server_resp::ServerResp;
pub use session::Session;
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
