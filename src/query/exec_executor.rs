use async_trait::async_trait;
use cassandra_proto::{
  error,
  frame::Frame,
  query::{QueryParams, QueryParamsBuilder, QueryValues},
  types::CBytesShort,
};

pub type PreparedQuery = CBytesShort;

#[async_trait]
pub trait ExecExecutor {
  async fn exec_with_params_tw(
    &self,
    prepared: &PreparedQuery,
    query_parameters: QueryParams,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame>;

  async fn exec_with_params(
    &self,
    prepared: &PreparedQuery,
    query_parameters: QueryParams,
  ) -> error::Result<Frame> {
    self
      .exec_with_params_tw(prepared, query_parameters, false, false)
      .await
  }

  async fn exec_with_values_tw<V: Into<QueryValues> + Send>(
    &self,
    prepared: &PreparedQuery,
    values: V,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let query_params_builder = QueryParamsBuilder::new();
    let query_params = query_params_builder.values(values.into()).finalize();
    self
      .exec_with_params_tw(prepared, query_params, with_tracing, with_warnings)
      .await
  }

  async fn exec_with_values<V: Into<QueryValues> + Send>(
    &self,
    prepared: &PreparedQuery,
    values: V,
  ) -> error::Result<Frame> {
    self
      .exec_with_values_tw(prepared, values, false, false)
      .await
  }

  async fn exec_tw(
    &self,
    prepared: &PreparedQuery,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let query_params = QueryParamsBuilder::new().finalize();
    self
      .exec_with_params_tw(prepared, query_params, with_tracing, with_warnings)
      .await
  }

  async fn exec(&self, prepared: &PreparedQuery) -> error::Result<Frame> {
    self.exec_tw(prepared, false, false).await
  }
}
