#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

mod utils_bootstrap;
mod utils_keyspace;
mod utils_session;

use async_std::task;
use std::pin::Pin;

use cdrs_async::query::QueryExecutor;
use cassandra_proto::{types::rows::Row, query::QueryValues};

async fn insert_large_record(executor: Pin<&mut impl QueryExecutor>) {
  let key = [0u8; 1].to_vec();
  let val = [1u8; 1_000].to_vec();
  executor.query_with_values(
    "INSERT INTO test_keyspace.test_table (key, value) VALUES (?, ?);",
    QueryValues::from(vec!(key, val)))
    .await
    .expect("could not select system local");
}

async fn select_from_test_table(executor: Pin<&mut impl QueryExecutor>) -> Vec<Row> {
  let key = [0u8; 1].to_vec();
  executor.query_with_values(
    "SELECT * FROM test_keyspace.test_table WHERE key = ?",
    QueryValues::from(vec!(key)))
    .await
    .expect("could not select from table")
    .get_body()
    .expect("could not obtain body from a response")
    .into_rows()
    .expect("could not get rows from a response")
}

speculate! {
  describe "QueryExecutor" {
    before {
      utils_bootstrap::bootstrap();
    }

    it "should run query" {
      task::block_on(async {
        let mut session = utils_session::connect_tcp().await;
        utils_keyspace::create_keyspace(Pin::new(&mut session)).await;
        utils_keyspace::create_table(Pin::new(&mut session)).await;
        insert_large_record(Pin::new(&mut session)).await;
        let rows = select_from_test_table(Pin::new(&mut session)).await;
        assert_eq!(rows.len(), 1, "should return exactly 1 row from the table");
      });
    }

    it "should run query with tracing and warnings" {
      // TODO:
    }

    it "should run query with values" {
      // TODO:
    }

    it "should run query with values with tracing and warnings" {
      // TODO:
    }

    it "should run query with params" {
      // TODO:
    }

    it "should run query with params with tracing and warnings" {
      // TODO:
    }
  }
}
