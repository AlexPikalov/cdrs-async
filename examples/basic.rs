extern crate async_std;
extern crate async_trait;
extern crate cdrs_async;

use async_std::pin::Pin;
use async_std::task;

use cdrs_async::{query::QueryExecutor, Session, TransportTcp};

const CREATE_KS_QUERY: &'static str = r#"
  CREATE KEYSPACE IF NOT EXIST async_cdrs_2
    WITH REPLICATION = { 
      'class' : 'SimpleStrategy', 
      'replication_factor' : 1 
    };
"#;

fn main() {
  task::block_on(async {
    let transport = TransportTcp::new("127.0.0.1:9042")
      .await
      .expect("create transport");
    let mut session = Session::from(transport);
    let pinned_session = Pin::new(&mut session);

    let r = pinned_session.query(CREATE_KS_QUERY).await;
    println!("Result {:?}", r);
  });
}
