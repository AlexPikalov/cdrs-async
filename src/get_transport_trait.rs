use std::sync::Arc;

use crate::transport::CDRSTransport;

/// `GetConnection` trait provides a unified interface for Session to get a connection
/// from a load balancer
pub trait GetTransport<T: CDRSTransport + Sized + Send + Sync + 'static> {
  /// Returns connection from a load balancer.
  fn get_transport(&self) -> Arc<T>;
}
