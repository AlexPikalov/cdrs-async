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

speculate! {
  describe "table" {
    const GET_TABLE_INFO_QUERY: &'static str = r#"
      SELECT * from system_schema.tables
        WHERE keyspace_name = 'test_keyspace'
        AND  table_name = 'test_table';
    "#;

    const DROP_TABLE_QUERY: &'static str = r#"
      DROP TABLE test_keyspace.test_table;
    "#; 

    before {
      utils_bootstrap::bootstrap();
    }

    it "should create and remove a table" {
      task::block_on(async {
        let mut session = utils_session::connect_tcp().await;
        // create a new keyspace
        utils_keyspace::create_keyspace(Pin::new(&mut session)).await;

        // create a new table
        utils_keyspace::create_table(Pin::new(&mut session)).await;

        // select an info about a table
        let keyspaces = Pin::new(&mut session)
          .query(GET_TABLE_INFO_QUERY)
          .await
          .expect("could not select table info")
          .get_body()
          .expect("could not obtain body from a response")
          .into_rows()
          .expect("could not get rows from a response");
        assert_eq!(keyspaces.len(), 1, "should create a table");

        // drop a table
        Pin::new(&mut session)
          .query(DROP_TABLE_QUERY)
          .await
          .expect("could not drop a table");

          // select an info about a table
        let keyspaces = Pin::new(&mut session)
          .query(GET_TABLE_INFO_QUERY)
          .await
          .expect("could not select table info")
          .get_body()
          .expect("could not obtain body from a response")
          .into_rows()
          .expect("could not get rows from a response");
        assert_eq!(keyspaces.len(), 0, "should drop a table");
      });
    }
  }
}
