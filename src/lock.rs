use crate::DbConn;
use diesel::prelude::*;

pub fn transaction_with_advisory_lock<F, R, E>(conn: &DbConn, lock: i64, f: F) -> Result<R, E>
where
  F: FnOnce() -> Result<R, E>,
  E: From<diesel::result::Error>,
{
  conn.transaction(|| {
    diesel::sql_query(r#"SELECT pg_advisory_xact_lock($1)"#)
      .bind::<diesel::sql_types::BigInt, _>(lock)
      .execute(conn)?;

    let res = f()?;
    Ok(res)
  })
}
