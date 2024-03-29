pub type Backend = diesel::pg::Pg;
pub type DbConn = diesel::pg::PgConnection;
pub type ConnectionManager = diesel::r2d2::ConnectionManager<DbConn>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type BoxedQuery<'a, Source> = <Source as diesel::query_dsl::methods::BoxedDsl<'a, Backend>>::Output;