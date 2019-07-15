use std::time::Duration;

use crate::config::DatabaseConfig;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::run_pending_migrations;

pub type PostgresConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_database(config: DatabaseConfig) -> PostgresConnectionPool {
    let db_pool = r2d2::Pool::builder()
        .max_size(config.pool_max_size)
        .min_idle(Some(config.pool_min_idle))
        .max_lifetime(Some(Duration::from_secs(config.pool_max_lifetime_seconds)))
        .build(ConnectionManager::<PgConnection>::new(config.url))
        .expect("Failed to initialize database connection!");

    db_pool
}

pub fn run_migrations(database: &PostgresConnectionPool) {
    let migration_connection = database.get().unwrap();
    run_pending_migrations(&migration_connection).expect("Database migrations failed");
}