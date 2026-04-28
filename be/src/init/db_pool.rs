use std::{fmt, time::Duration};

use diesel_async::{
    AsyncConnection, AsyncMigrationHarness, AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool, bb8::PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tracing::info;

use crate::init::server_config::db_config::{DatabaseConfig, DatabaseConnectionType, DatabaseType};

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConnection<'a> = PooledConnection<'a, AsyncPgConnection>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Debug)]
pub enum DbPoolInitError {
    UnsupportedDatabase { database_type: DatabaseType },
    Build { error: String },
    GetConnection { error: String },
    Migrate { error: String },
}

impl fmt::Display for DbPoolInitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedDatabase { database_type } => {
                write!(
                    formatter,
                    "database backend `{database_type}` is not implemented; PostgreSQL is required"
                )
            }
            Self::Build { error } => write!(formatter, "failed to build database pool: {error}"),
            Self::GetConnection { error } => {
                write!(formatter, "failed to get database connection: {error}")
            }
            Self::Migrate { error } => write!(formatter, "failed to run migrations: {error}"),
        }
    }
}

pub async fn build_db_pool(db_config: &DatabaseConfig) -> Result<DbPool, DbPoolInitError> {
    match db_config.database_type {
        DatabaseType::Postgres => {}
        DatabaseType::MySql | DatabaseType::Sqlite => {
            return Err(DbPoolInitError::UnsupportedDatabase {
                database_type: db_config.database_type,
            });
        }
    }

    let connection_string = db_config.postgres_connection_string();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_string);
    let physical_cores = physical_parallelism();
    let max_size = physical_cores.saturating_mul(10);

    let pool = match Pool::builder()
        .min_idle(Some(physical_cores))
        .max_size(max_size)
        .connection_timeout(Duration::from_secs(2))
        .build(manager)
        .await
    {
        Ok(pool) => pool,
        Err(error) => {
            return Err(DbPoolInitError::Build {
                error: error.to_string(),
            });
        }
    };

    Ok(pool)
}

pub async fn run_db_migrations(db_config: &DatabaseConfig) -> Result<(), DbPoolInitError> {
    match db_config.database_type {
        DatabaseType::Postgres => {}
        DatabaseType::MySql | DatabaseType::Sqlite => {
            return Err(DbPoolInitError::UnsupportedDatabase {
                database_type: db_config.database_type,
            });
        }
    }

    info!(
        database_type = %db_config.database_type,
        "Running database migrations"
    );

    let connection =
        match AsyncPgConnection::establish(&db_config.postgres_connection_string()).await {
            Ok(connection) => connection,
            Err(error) => {
                return Err(DbPoolInitError::GetConnection {
                    error: error.to_string(),
                });
            }
        };

    let mut harness = AsyncMigrationHarness::new(connection);
    match harness.run_pending_migrations(MIGRATIONS) {
        Ok(applied_migrations) => {
            for migration_version in &applied_migrations {
                info!(
                    migration_version = %migration_version,
                    "Applied database migration"
                );
            }

            info!(
                database_type = %db_config.database_type,
                applied_migration_count = applied_migrations.len(),
                "Database migrations complete"
            );

            Ok(())
        }
        Err(error) => Err(DbPoolInitError::Migrate {
            error: error.to_string(),
        }),
    }
}

pub async fn get_conn(pool: &DbPool) -> Result<DbConnection<'_>, DbPoolInitError> {
    match pool.get().await {
        Ok(connection) => Ok(connection),
        Err(error) => Err(DbPoolInitError::GetConnection {
            error: error.to_string(),
        }),
    }
}

fn physical_parallelism() -> u32 {
    let parallelism = match std::thread::available_parallelism() {
        Ok(value) => value.get(),
        Err(_) => 4,
    };

    if parallelism > u32::MAX as usize {
        return u32::MAX / 10;
    }

    parallelism as u32
}

impl DatabaseConfig {
    pub fn postgres_connection_string(&self) -> String {
        match self.database_connection_type {
            DatabaseConnectionType::Local
            | DatabaseConnectionType::Remote
            | DatabaseConnectionType::DomainSocket => format!(
                "host={} port={} user={} password={} dbname={}",
                postgres_value(&self.database_host),
                self.database_port,
                postgres_value(&self.database_username),
                postgres_value(&self.database_password),
                postgres_value(&self.database_name)
            ),
        }
    }
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Postgres => formatter.write_str("postgres"),
            Self::MySql => formatter.write_str("mysql"),
            Self::Sqlite => formatter.write_str("sqlite"),
        }
    }
}

fn postgres_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('\'');
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '\'' => escaped.push_str("\\'"),
            _ => escaped.push(character),
        }
    }
    escaped.push('\'');
    escaped
}
