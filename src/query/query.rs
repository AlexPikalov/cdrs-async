use super::QueryParams;

/// Structure that represents CQL query and parameters which will be applied during
/// its execution.
#[derive(Debug, Default)]
pub struct Query {
    /// Query string.
    pub query: String,
    /// Parameters of query.
    pub params: QueryParams,
}
