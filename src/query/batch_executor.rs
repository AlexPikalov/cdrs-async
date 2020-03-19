use async_trait::async_trait;
use cassandra_proto::{error, frame::Frame, query::QueryBatch};

#[async_trait]
pub trait BatchExecutor {
  async fn batch_with_params_tw(
    &self,
    batch: QueryBatch,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame>;

  async fn batch_with_params(&self, batch: QueryBatch) -> error::Result<Frame> {
    self.batch_with_params_tw(batch, false, false).await
  }
}
