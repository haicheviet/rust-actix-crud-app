use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
