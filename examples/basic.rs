extern crate async_std;
extern crate async_trait;
extern crate cdrs_async;

use std::pin::Pin;

use async_std::task;
use cdrs_async::{
    authenticators::NoneAuthenticator, query::QueryExecutor, Compression, Session, TransportTcp,
};

const CREATE_KS_QUERY: &'static str = r#"
  CREATE KEYSPACE IF NOT EXISTS async_cdrs_3
    WITH REPLICATION = { 
      'class' : 'SimpleStrategy', 
      'replication_factor' : 1 
    };
"#;

fn main() {
    task::block_on(async {
        let authenticator_strategy = NoneAuthenticator {};
        let transport = TransportTcp::new("127.0.0.1:9042")
            .await
            .expect("Cannot create transport");
        let mut session =
            Session::connect(transport, Compression::None, authenticator_strategy.into())
                .await
                .expect("session connect");
        let pinned_session = Pin::new(&mut session);

        let r = pinned_session.query(CREATE_KS_QUERY).await;
        println!("Result {:?}", r);
    });
}
