use diesel;
use std::future::Future;
use std::sync::Arc;
use std::fmt;
use thiserror::Error;

use crate::result::{DbError, Result};
use crate::types::{ConnectionManager, DbConn, Pool};
use tokio::runtime::Handle;

pub struct Executor {
  db_conn_pool: Pool,
}

impl fmt::Debug for Executor {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Executor")
       .finish()
  }
}

impl Executor {
  pub fn new(db_url: String) -> Self {
    let manager = ConnectionManager::new(db_url);
    let pool = diesel::r2d2::Pool::builder()
      .build(manager)
      .expect("error to create connection pool");
    Executor { db_conn_pool: pool }
  }

  pub fn into_ref(self) -> ExecutorRef {
    Arc::new(self)
  }

  pub fn env() -> Self {
    ::dotenv::dotenv().ok();
    use crate::get_env_database_url;
    let database_url = get_env_database_url();
    Self::new(database_url)
  }

  pub fn exec<F, T, E>(&self, f: F) -> impl Future<Output = Result<T, ExecutorError<E>>>
  where
    F: FnOnce(&DbConn) -> Result<T, E> + Send + 'static,
    T: Send + 'static,
    E: Send + std::fmt::Debug + std::fmt::Display + 'static,
  {
    let pool = self.db_conn_pool.clone();
    async move {
      let handle = Handle::try_current().map_err(DbError::NoRuntime)?;
      handle
        .enter(move || {
          tokio::task::spawn_blocking(move || -> Result<T, ExecutorError<E>> {
            let conn = pool.get().map_err(DbError::R2d2Pool)?;
            f(&conn).map_err(ExecutorError::Task)
          })
        })
        .await
        .map_err(DbError::Task)?
    }
  }
}

pub type ExecutorRef = Arc<Executor>;

#[derive(Error, Debug)]
pub enum ExecutorError<E>
where
  E: std::fmt::Debug + std::fmt::Display,
{
  #[error("executor: {0}")]
  Executor(#[from] DbError),
  #[error("{0}")]
  Task(E),
}
