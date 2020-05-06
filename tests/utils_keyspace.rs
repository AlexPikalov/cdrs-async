use std::pin::Pin;

use cdrs_async::query::QueryExecutor;

pub const CREATE_KS_QUERY: &'static str = r#"
  CREATE KEYSPACE IF NOT EXISTS test_keyspace
    WITH REPLICATION = { 
      'class' : 'SimpleStrategy', 
      'replication_factor' : 1 
    };
  "#;

pub const DROP_KS_QUERY: &'static str = r#"
  DROP KEYSPACE IF EXISTS test_keyspace;
  "#;

pub async fn create_keyspace(executor: Pin<&mut impl QueryExecutor>) {
    executor
        .query(CREATE_KS_QUERY)
        .await
        .expect("should create test_keyspace");
}

pub async fn drop_keyspace(executor: Pin<&mut impl QueryExecutor>) {
    executor
        .query(DROP_KS_QUERY)
        .await
        .expect("should drop test_keyspace");
}
