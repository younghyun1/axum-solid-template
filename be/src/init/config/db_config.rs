#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    MySql,
    Sqlite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseConnectionType {
    Local,
    Remote,
    DomainSocket,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub database_connection_type: DatabaseConnectionType,
    pub database_host: String,
    pub database_port: u16,
    pub database_username: String,
    pub database_password: String,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn new(
        database_type: DatabaseType,
        database_connection_type: DatabaseConnectionType,
        database_host: String,
        database_port: u16,
        database_username: String,
        database_password: String,
        database_name: String,
    ) -> Self {
        Self {
            database_type,
            database_connection_type,
            database_host,
            database_port,
            database_username,
            database_password,
            database_name,
        }
    }
}
