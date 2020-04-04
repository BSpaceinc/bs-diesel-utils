pub type DbConn = diesel::pg::PgConnection;
pub type ConnectionManager = diesel::r2d2::ConnectionManager<DbConn>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
