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
    database_type: DatabaseType,
    database_connection_type: DatabaseConnectionType,
    database_host: String,
    database_port: u16,
    database_username: String,
    database_password: String,
    database_name: String,
}
