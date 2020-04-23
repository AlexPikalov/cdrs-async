use cdrs_async::{authenticators::NoneAuthenticator, Compression, Session, TransportTcp};

pub async fn connect_tcp() -> Session<TransportTcp> {
  let authenticator_strategy = NoneAuthenticator {};
  Session::connect(
    "127.0.0.1:9042",
    Compression::None,
    authenticator_strategy.into(),
  )
  .await
  .expect("session connect")
}
