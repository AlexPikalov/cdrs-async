# CDRS-async - async Apache Cassandra and Scylla driver for Rust

<p align="center">
  <img src="https://github.com/AlexPikalov/cdrs/raw/master/cdrs-logo.png" alt="CDRS - Apache Cassandra driver"/>
</p>

CDRS-async is an asynchronous driver for Apache Cassandra and Scylla databases.

It's still alpha but you can try it out.

## Features

- LZ4, Snappy compression;

- Cassandra-to-Rust data deserialization;

- Pluggable authentication strategies;

- ScyllaDB support;

- Server events listening;

- Multiple CQL version support (3, 4), full spec implementation;

- Query tracing information.

## Getting started

Add CDRS-async to your Cargo.toml file as a dependency:

```toml
cdrs-async = { version = "*" }
```
The library supports both `tokio` and `async_std` by default. You can enable and disable them through the features `async_std`, `tok_io` and `no_runtime`. If you wish to use a different runtime, you have to implement the trait `CDRSTransport` and pass an object of this type to the session.


Then add it as an external crate to your `main.rs` file:

```rust
extern crate async_std;
extern crate async_trait;
extern crate cdrs_async;

use async_std::{pin::Pin, task};
use cdrs_async::{authenticators::NoneAuthenticator, query::QueryExecutor, Compression, Session, transport::async_std::TransportTcp};

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
    let mut session = Session::connect(
      transport,
      Compression::None,
      authenticator_strategy.into(),
    )
    .await
    .expect("session connect");
    let pinned_session = Pin::new(&mut session);

    let r = pinned_session.query(CREATE_KS_QUERY).await;
    println!("Result {:?}", r);
  });
}
```

## License

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.
