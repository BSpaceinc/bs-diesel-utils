use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
  #[error("diesel: {0}")]
  Diesel(#[from] diesel::result::Error),
  #[error("r2d2 connection pool: {0}")]
  R2d2Pool(#[from] diesel::r2d2::PoolError),
  #[error("no running async runtime: {0}")]
  NoRuntime(#[from] tokio::runtime::TryCurrentError),
  #[error("task failed to execute to completion: {0}")]
  Task(#[from] tokio::task::JoinError),
}

pub type Result<T, E = DbError> = std::result::Result<T, E>;
