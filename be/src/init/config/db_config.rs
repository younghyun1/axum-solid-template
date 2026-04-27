enum DatabaseType {
    Postgres,
    MySql,
    Sqlite,
}

enum DatabaseConnectionType {
    Local,
    Remote,
    DomainSocket,
}

pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub database_connection_type: DatabaseConnectionType,
    pub database_host: String,
    pub database_port: u16,
    pub database_username: String,
    pub database_password: String,
    pub database_name: String,
}
