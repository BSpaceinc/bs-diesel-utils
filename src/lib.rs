pub use bigdecimal::BigDecimal;
pub use diesel;

mod environment;
mod types;

pub mod executor;
pub mod result;

pub use self::executor::{Executor, ExecutorRef};
pub use self::result::DbError;
pub use self::types::*;

use environment::Environment;

/// Gets database url from env
///
/// Either `DATABASE_URL` or combination of
///   `DB_USER`, `DB_PASSWORD`, `DB_HOST`, `DB_DATABASE` envs must be set
///
/// When `ENVIRONMENT`=`TEST`
///   expects user to set `DATABASE_TEST_URL` env
pub fn get_env_database_url() -> String {
  use std::env;

  if let Ok(env_type) = env::var("ENVIRONMENT") {
    let env_type = Environment::new(env_type);
    if env_type.is_test() {
      return env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");
    }
  }

  match env::var("DATABASE_URL").ok() {
    Some(url) => url,
    None => format!(
      "postgres://{}:{}@{}/{}",
      env::var("DB_USER").unwrap(),
      env::var("DB_PASSWORD").unwrap(),
      env::var("DB_HOST").unwrap(),
      env::var("DB_DATABASE").unwrap(),
    ),
  }
}

/// For test and cli use only
///
/// # Panics
///
/// Panics if connection failed.
///
pub fn connect_env() -> DbConn {
  use diesel::prelude::*;

  let database_url = get_env_database_url();
  let conn =
    DbConn::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
  conn
}

/// Starts test transaction
///
pub fn start_test_transaction(conn: DbConn) -> DbConn {
  use diesel::prelude::*;
  conn
    .begin_test_transaction()
    .expect("begin test transaction");
  conn
}
