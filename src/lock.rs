use crate::DbConn;
use diesel::prelude::*;

pub trait AsAdvisoryLockID {
  fn as_advisory_lock_id(&self) -> i64;
}

impl AsAdvisoryLockID for i64 {
  fn as_advisory_lock_id(&self) -> i64 {
    *self
  }
}

pub fn transaction_with_advisory_lock<ID, F, R, E>(conn: &DbConn, lock_id: ID, f: F) -> Result<R, E>
where
  ID: AsAdvisoryLockID,
  F: FnOnce() -> Result<R, E>,
  E: From<diesel::result::Error>,
{
  conn.transaction(|| {
    diesel::sql_query(r#"SELECT pg_advisory_xact_lock($1)"#)
      .bind::<diesel::sql_types::BigInt, _>(lock_id.as_advisory_lock_id())
      .execute(conn)?;

    let res = f()?;
    Ok(res)
  })
}
