mod batch_executor;
mod exec_executor;
mod prepare_executor;
mod query;
mod query_executor;
mod query_flags;
mod query_params;
mod query_values;

pub use batch_executor::BatchExecutor;
pub use exec_executor::ExecExecutor;
pub use prepare_executor::{PrepareExecutor, PreparedQuery};
pub use query::Query;
pub use query_executor::QueryExecutor;
pub use query_flags::QueryFlags;
pub use query_params::QueryParams;
pub use query_values::QueryValues;

use crate::compressor::Compression;

pub(crate) trait GetCompression {
  fn get_compression(&self) -> &Compression;
}
