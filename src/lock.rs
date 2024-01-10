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

pub fn transaction_with_advisory_lock<ID, F, R, E>(conn: &mut DbConn, lock_id: ID, f: F) -> Result<R, E>
where
  ID: AsAdvisoryLockID,
  F: FnOnce( &mut DbConn) -> Result<R, E>,
  E: From<diesel::result::Error>,
{
  conn.transaction(|conn| {
    diesel::sql_query(r#"SELECT pg_advisory_xact_lock($1)"#)
      .bind::<diesel::sql_types::BigInt, _>(lock_id.as_advisory_lock_id())
      .execute(conn)?;

    let res = f(conn)?;
    Ok(res)
  })
}

pub fn transaction_with_advisory_locks<ID, F, R, E>(
  conn: &mut DbConn,
  lock_ids: &[ID],
  f: F,
) -> Result<R, E>
where
  ID: AsAdvisoryLockID,
  F: FnOnce() -> Result<R, E>,
  E: From<diesel::result::Error>,
{
  conn.transaction(|conn| {
    if !lock_ids.is_empty() {
      let mut sql = "SELECT ".to_string();
      for (idx, id) in lock_ids.into_iter().enumerate() {
        if idx > 0 {
          sql.push_str(",")
        }
        sql.push_str(&format!(
          "pg_advisory_xact_lock({})",
          id.as_advisory_lock_id()
        ));
      }
      diesel::sql_query(sql).execute(conn)?;
    }

    let res = f()?;
    Ok(res)
  })
}
