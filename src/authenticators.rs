use cassandra_proto::types::CBytes;

pub struct Authenticator {
  cassandra_name: Option<String>,
  auth_token: CBytes,
}

impl Authenticator {
  pub fn get_auth_token(&self) -> CBytes {
    self.auth_token.clone()
  }
  pub fn get_cassandra_name(&self) -> Option<String> {
    self.cassandra_name.clone()
  }
}

#[derive(Debug, Clone)]
pub struct PasswordAuthenticator {
  username: String,
  password: String,
}

impl PasswordAuthenticator {
  pub fn new<S: ToString>(username: S, password: S) -> PasswordAuthenticator {
    PasswordAuthenticator {
      username: username.to_string(),
      password: password.to_string(),
    }
  }
}

impl Into<Authenticator> for PasswordAuthenticator {
  fn into(self) -> Authenticator {
    let auth_token = {
      let mut v = vec![0];
      v.extend_from_slice(self.username.as_bytes());
      v.push(0);
      v.extend_from_slice(self.password.as_bytes());

      CBytes::new(v)
    };

    Authenticator {
      cassandra_name: Some("org.apache.cassandra.auth.PasswordAuthenticator".into()),
      auth_token,
    }
  }
}

#[derive(Debug, Clone)]
pub struct NoneAuthenticator;

impl Into<Authenticator> for NoneAuthenticator {
  fn into(self) -> Authenticator {
    Authenticator {
      cassandra_name: None,
      auth_token: CBytes::new(vec![0]),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_password_authenticator_trait_impl() {
    let authenticator = PasswordAuthenticator::new("a", "a");
    let _ = authenticator_tester(Box::new(authenticator.into()));
  }

  #[test]
  fn test_password_authenticator_new() {
    PasswordAuthenticator::new("foo", "bar");
  }

  #[test]
  fn test_password_authenticator_get_cassandra_name() {
    let auth: Authenticator = PasswordAuthenticator::new("foo", "bar").into();
    assert_eq!(
      auth.get_cassandra_name(),
      Some("org.apache.cassandra.auth.PasswordAuthenticator".into())
    );
  }

  #[test]
  fn test_password_authenticator_get_auth_token() {
    let auth: Authenticator = PasswordAuthenticator::new("foo", "bar").into();
    let mut expected_token = vec![0];
    expected_token.extend_from_slice("foo".as_bytes());
    expected_token.push(0);
    expected_token.extend_from_slice("bar".as_bytes());

    assert_eq!(auth.get_auth_token().into_plain().unwrap(), expected_token);
  }

  #[test]
  fn test_authenticator_none_get_cassandra_name() {
    let auth: Authenticator = (NoneAuthenticator {}).into();
    assert_eq!(auth.get_cassandra_name(), None);
    assert_eq!(auth.get_auth_token().into_plain().unwrap(), vec![0]);
  }

  fn authenticator_tester(_authenticator: Box<Authenticator>) {}
}
