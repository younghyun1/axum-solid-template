use std::fmt;

#[derive(Debug)]
pub enum MarketplacePublicCacheError {
    Io { error: String },
    Serialization { error: String },
    TaskJoin { error: String },
}

pub fn io_error(error: impl fmt::Display) -> MarketplacePublicCacheError {
    MarketplacePublicCacheError::Io {
        error: error.to_string(),
    }
}

pub fn serialization_error(error: impl fmt::Display) -> MarketplacePublicCacheError {
    MarketplacePublicCacheError::Serialization {
        error: error.to_string(),
    }
}

impl fmt::Display for MarketplacePublicCacheError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { error } => write!(formatter, "marketplace cache I/O error: {error}"),
            Self::Serialization { error } => {
                write!(formatter, "marketplace cache serialization error: {error}")
            }
            Self::TaskJoin { error } => {
                write!(formatter, "marketplace cache task join error: {error}")
            }
        }
    }
}
