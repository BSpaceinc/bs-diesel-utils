use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
  #[error("diesel: {0}")]
  Diesel(#[from] diesel::result::Error),
  #[error("r2d2 connection pool: {0}")]
  R2d2Pool(#[from] diesel::r2d2::PoolError),
  #[error("no running async runtime")]
  NoRuntime,
  #[error("task failed to execute to completion: {0}")]
  Task(#[from] tokio::task::JoinError),
}

pub type Result<T, E = DbError> = std::result::Result<T, E>;

pub trait DieselErrorExt {
  fn is_unique_violation(&self, constraint_name: &str) -> bool;
  fn is_foreign_key_violation(&self, constraint_name: Option<&str>) -> bool;
}

impl DieselErrorExt for diesel::result::Error {
  fn is_unique_violation(&self, constraint_name: &str) -> bool {
    use diesel::result::{DatabaseErrorKind, Error};
    match *self {
      Error::DatabaseError(DatabaseErrorKind::UniqueViolation, ref info) => {
        info.constraint_name() == Some(constraint_name)
      }
      _ => false,
    }
  }

  fn is_foreign_key_violation(&self, constraint_name: Option<&str>) -> bool {
    use diesel::result::{DatabaseErrorKind, Error};
    match *self {
      Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, ref info) => {
        constraint_name.is_none() || info.constraint_name() == constraint_name
      }
      _ => false,
    }
  }
}
