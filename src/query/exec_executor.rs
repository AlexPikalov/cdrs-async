use std::pin::Pin;

use async_trait::async_trait;
use cassandra_proto::{
  error,
  frame::Frame,
  query::{QueryParams, QueryParamsBuilder, QueryValues},
  types::CBytesShort,
};

/// Prepared query ID.
pub type PreparedQuery = CBytesShort;

/// Traits that provides methods for prepared query execution.
#[async_trait]
pub trait ExecExecutor: Send {
  async fn exec_with_params_tw(
    mut self: Pin<&mut Self>,
    prepared: &PreparedQuery,
    query_parameters: QueryParams,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame>;

  async fn exec_with_params(
    mut self: Pin<&mut Self>,
    prepared: &PreparedQuery,
    query_parameters: QueryParams,
  ) -> error::Result<Frame> {
    self
      .exec_with_params_tw(prepared, query_parameters, false, false)
      .await
  }

  async fn exec_with_values_tw<V: Into<QueryValues> + Send>(
    mut self: Pin<&mut Self>,
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
    mut self: Pin<&mut Self>,
    prepared: &PreparedQuery,
    values: V,
  ) -> error::Result<Frame> {
    self
      .exec_with_values_tw(prepared, values, false, false)
      .await
  }

  async fn exec_tw(
    mut self: Pin<&mut Self>,
    prepared: &PreparedQuery,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let query_params = QueryParamsBuilder::new().finalize();
    self
      .exec_with_params_tw(prepared, query_params, with_tracing, with_warnings)
      .await
  }

  async fn exec(mut self: Pin<&mut Self>, prepared: &PreparedQuery) -> error::Result<Frame> {
    self.exec_tw(prepared, false, false).await
  }
}
