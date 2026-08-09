use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::{sync::OnceLock, time::Duration};

static DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init_db() -> Result<(), DbErr> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(20)
       .min_connections(2)
       .connect_timeout(Duration::from_secs(5))
       .acquire_timeout(Duration::from_secs(5))
       .idle_timeout(Duration::from_secs(600))
       .max_lifetime(Duration::from_secs(1800))
       .sqlx_logging(true);

    let connection = Database::connect(opt).await.expect("Error on Connection");
    
    DB_CONNECTION.set(connection).expect("Database already initialized");
    Ok(())
}

pub fn get_db_connection() -> &'static DatabaseConnection {
    DB_CONNECTION.get().expect("Database not initialized. Call init_db() first.")
}