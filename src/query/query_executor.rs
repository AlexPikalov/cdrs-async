use std::sync::Arc;

use async_std::prelude::*;
use async_trait::async_trait;
use futures::FutureExt;

use crate::transport::CDRSTransport;
use crate::{GetCompressor, GetTransport};
use cassandra_proto::{
  error,
  frame::{Flag, Frame, IntoBytes},
  query::{Query, QueryParams, QueryParamsBuilder, QueryValues},
};

#[async_trait]
pub trait QueryExecutor {
  async fn query_with_params_tw<Q: ToString + Send>(
    &self,
    query: Q,
    query_params: QueryParams,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    // build query
    //
    unimplemented!()
  }

  /// Executes a query with default parameters:
  /// * TDB
  async fn query<Q: ToString + Send>(&self, query: Q) -> error::Result<Frame> {
    self.query_tw(query, false, false).await
  }

  /// Executes a query with ability to trace it and see warnings, and default parameters:
  /// * TBD
  async fn query_tw<Q: ToString + Send>(
    &self,
    query: Q,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let query_params = QueryParamsBuilder::new().finalize();
    self
      .query_with_params_tw(query, query_params, with_tracing, with_warnings)
      .await
  }

  /// Executes a query with bounded values (either with or without names).
  async fn query_with_values<Q: ToString + Send, V: Into<QueryValues> + Send>(
    &self,
    query: Q,
    values: V,
  ) -> error::Result<Frame> {
    self.query_with_values_tw(query, values, false, false).await
  }

  /// Executes a query with bounded values (either with or without names)
  /// and ability to see warnings, trace a request and default parameters.
  async fn query_with_values_tw<Q: ToString + Send, V: Into<QueryValues> + Send>(
    &self,
    query: Q,
    values: V,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<Frame> {
    let query_params_builder = QueryParamsBuilder::new();
    let query_params = query_params_builder.values(values.into()).finalize();
    self
      .query_with_params_tw(query, query_params, with_tracing, with_warnings)
      .await
  }

  /// Executes a query with query params without warnings and tracing.
  async fn query_with_params<Q: ToString + Send>(
    &self,
    query: Q,
    query_params: QueryParams,
  ) -> error::Result<Frame> {
    self
      .query_with_params_tw(query, query_params, false, false)
      .await
  }
}
