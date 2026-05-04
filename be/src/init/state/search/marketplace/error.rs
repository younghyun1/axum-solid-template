use std::fmt;

use crate::init::db_pool::DbPoolInitError;

#[derive(Debug)]
pub enum MarketplaceSearchError {
    DbPool { error: DbPoolInitError },
    Query { error: String },
    Index { error: String },
    TaskJoin { error: String },
}

pub fn query_error(error: impl fmt::Display) -> MarketplaceSearchError {
    MarketplaceSearchError::Query {
        error: error.to_string(),
    }
}

pub fn index_error(error: impl fmt::Display) -> MarketplaceSearchError {
    MarketplaceSearchError::Index {
        error: error.to_string(),
    }
}

impl fmt::Display for MarketplaceSearchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbPool { error } => write!(formatter, "search database pool error: {error}"),
            Self::Query { error } => write!(formatter, "search source query error: {error}"),
            Self::Index { error } => write!(formatter, "search index error: {error}"),
            Self::TaskJoin { error } => write!(formatter, "search task join error: {error}"),
        }
    }
}
