extern crate async_std;
extern crate async_trait;
extern crate cdrs_async;

use async_std::pin::Pin;
use async_std::task;

use cdrs_async::{authenticators::NoneAuthenticator, query::QueryExecutor, Compression, Session};

const CREATE_KS_QUERY: &'static str = r#"
  CREATE KEYSPACE IF NOT EXISTS async_cdrs_2
    WITH REPLICATION = { 
      'class' : 'SimpleStrategy', 
      'replication_factor' : 1 
    };
"#;

fn main() {
  task::block_on(async {
    let mut session = Session::connect("127.0.0.1:9042", Compression::None, NoneAuthenticator {})
      .await
      .expect("session connect");
    let pinned_session = Pin::new(&mut session);

    let r = pinned_session.query(CREATE_KS_QUERY).await;
    println!("Result {:?}", r);
  });
}
