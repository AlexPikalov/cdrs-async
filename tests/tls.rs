#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

use async_std::task;

speculate! {
    describe "TransportTls" {
        it "should be able to connect to github.com:443" {
            task::block_on(async {
                assert!(cdrs_async::TransportTls::new("github.com:443", Default::default()).await.is_ok());
            })
        }
    }
}
