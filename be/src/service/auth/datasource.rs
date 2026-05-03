use crate::{
    dto::api_response::ApiResult,
    error::{api_error::ApiError, code_error::CodeError},
    init::{
        db_pool::DbConnection, server_config::db_config::DatabaseType,
        state::server_state::ServerState,
    },
};

/// Perform the `postgres_conn` operation as implemented by this function.
///
/// # Arguments
/// * `state` -
/// # Returns
/// A `Result`, either containing the function output or an error.
pub async fn postgres_conn(state: &ServerState) -> ApiResult<DbConnection<'_>> {
    match state.server_config.db_config.database_type {
        DatabaseType::Postgres => {}
        DatabaseType::MySql | DatabaseType::Sqlite => {
            return Err(ApiError::new(CodeError::DATABASE_UNSUPPORTED));
        }
    }

    match state.get_conn().await {
        Ok(conn) => Ok(conn),
        Err(error) => Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error)),
    }
}
