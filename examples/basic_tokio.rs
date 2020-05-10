use std::pin::Pin;

use cdrs_async::{
    authenticators::NoneAuthenticator, query::QueryExecutor, Compression, Session, transport::tokio::TransportTcp,
};

const CREATE_KS_QUERY: &'static str = r#"
  CREATE KEYSPACE IF NOT EXISTS async_cdrs_tokio
    WITH REPLICATION = { 
      'class' : 'SimpleStrategy', 
      'replication_factor' : 1 
    };
"#;

#[tokio::main]
async fn main() {
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
}
