pub use bigdecimal::BigDecimal;
pub use diesel;

mod types;

pub mod executor;
pub mod result;

pub use self::executor::{Executor, ExecutorRef};
pub use self::result::DbError;
pub use self::types::*;

pub fn get_env_database_url() -> String {
  use std::env;
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
