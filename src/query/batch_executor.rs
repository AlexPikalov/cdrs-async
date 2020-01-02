use std::cell::RefCell;

use async_trait::async_trait;

use crate::transport::CDRSTransport;
use crate::{GetCompressor, GetTransport};
use cassandra_proto::{
  error,
  frame::traits::IntoBytes,
  frame::{Flag, Frame},
  query::QueryBatch,
};

#[async_trait]
pub trait BatchExecutor {
  async fn batch_with_params_tw(
    &self,
    batch: QueryBatch,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    unimplemented!();
  }

  async fn batch_with_params(&self, batch: QueryBatch) -> error::Result<Frame> {
    self.batch_with_params_tw(batch, false, false).await
  }
}
