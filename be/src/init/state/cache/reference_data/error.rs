use std::fmt;

use crate::init::db_pool::DbPoolInitError;

#[derive(Debug)]
pub enum ReferenceDataCacheError {
    DbPool {
        table: &'static str,
        error: DbPoolInitError,
    },
    Query {
        table: &'static str,
        error: String,
    },
    DuplicateIndex {
        index: &'static str,
    },
    TaskJoin {
        task: &'static str,
        error: String,
    },
}

impl fmt::Display for ReferenceDataCacheError {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbPool { table, error } => {
                write!(
                    formatter,
                    "failed to get database connection for `{table}`: {error}"
                )
            }
            Self::Query { table, error } => {
                write!(
                    formatter,
                    "failed to load `{table}` reference data: {error}"
                )
            }
            Self::DuplicateIndex { index } => {
                write!(formatter, "duplicate reference data cache index `{index}`")
            }
            Self::TaskJoin { task, error } => {
                write!(
                    formatter,
                    "reference data cache task `{task}` failed: {error}"
                )
            }
        }
    }
}
