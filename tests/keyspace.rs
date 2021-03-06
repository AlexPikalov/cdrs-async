#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

mod utils_bootstrap;
mod utils_keyspace;
mod utils_session;

use std::pin::Pin;

use async_std::task;

use cdrs_async::query::QueryExecutor;
use utils_bootstrap::bootstrap;
use utils_session::connect_tcp;

speculate! {
  describe "keyspace" {
    const SELECT_KS_NAMES_QUERY: &'static str = r#"
      SELECT * from system_schema.keyspaces
        WHERE keyspace_name = 'test_keyspace';
    "#;

    before {
      bootstrap();
    }

    it "should create a new keyspace" {
      task::block_on(async {
        let mut session = connect_tcp().await;
        // create a new keyspace
        utils_keyspace::create_keyspace(Pin::new(&mut session)).await;
        // select all existing keyspaces
        let keyspaces = Pin::new(&mut session)
          .query(SELECT_KS_NAMES_QUERY)
          .await
          .expect("could not select keyspaces")
          .get_body()
          .expect("could not obtain body from a response")
          .into_rows()
          .expect("could not get rows from a response");
        assert_eq!(keyspaces.len(), 1, "should create a keyspace");});
    }

    it "should remove a keyspace" {
      task::block_on(async {
        let mut session = connect_tcp().await;

        // create a new keyspace
        utils_keyspace::create_keyspace(Pin::new(&mut session)).await;

        // select all existing keyspaces
        let keyspaces = Pin::new(&mut session)
          .query(SELECT_KS_NAMES_QUERY)
          .await
          .expect("could not select keyspaces")
          .get_body()
          .expect("could not obtain body from a response")
          .into_rows()
          .expect("could not get rows from a response");

        assert_eq!(keyspaces.len(), 1, "should create a keyspace");

        // drop the keyspace
        utils_keyspace::drop_keyspace(Pin::new(&mut session)).await;

        // select all existing keyspaces
        let keyspaces = Pin::new(&mut session)
          .query(SELECT_KS_NAMES_QUERY)
          .await
          .expect("could not select keyspaces")
          .get_body()
          .expect("could not obtain body from a response")
          .into_rows()
          .expect("could not get rows from a response");

        assert_eq!(keyspaces.len(), 0, "should create a keyspace");
      });
    }
  }
}
