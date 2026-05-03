use std::fmt;

use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::{SaltString, rand_core::OsRng},
};
use zeroize::Zeroize;

#[derive(Debug)]
pub enum PasswordCryptoError {
    Params { error: String },
    Hash { error: String },
    Parse { error: String },
    Verify { error: String },
    Join { error: String },
}

impl fmt::Display for PasswordCryptoError {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Params { error } => write!(formatter, "failed to configure Argon2id: {error}"),
            Self::Hash { error } => write!(formatter, "failed to hash password: {error}"),
            Self::Parse { error } => write!(formatter, "failed to parse password hash: {error}"),
            Self::Verify { error } => write!(formatter, "failed to verify password: {error}"),
            Self::Join { error } => write!(formatter, "password task failed: {error}"),
        }
    }
}

/// Perform the `hash_password` operation as implemented by this function.
///
/// # Arguments
/// * `password` -
/// # Returns
/// A `Result`, either containing the function output or an error.
pub async fn hash_password(password: String) -> Result<String, PasswordCryptoError> {
    let join_result = tokio::task::spawn_blocking(move || {
        let mut password = password;
        let argon2 = match strong_argon2() {
            Ok(argon2) => argon2,
            Err(error) => {
                password.zeroize();
                return Err(error);
            }
        };
        let salt = SaltString::generate(&mut OsRng);

        let result = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(password_hash) => Ok(password_hash.to_string()),
            Err(error) => Err(PasswordCryptoError::Hash {
                error: error.to_string(),
            }),
        };
        password.zeroize();
        result
    })
    .await;

    match join_result {
        Ok(result) => result,
        Err(error) => Err(PasswordCryptoError::Join {
            error: error.to_string(),
        }),
    }
}

pub async fn verify_password(
    password: String,
    expected_hash: String,
) -> Result<bool, PasswordCryptoError> {
    let join_result = tokio::task::spawn_blocking(move || {
        let mut password = password;
        let mut expected_hash = expected_hash;
        let argon2 = match strong_argon2() {
            Ok(argon2) => argon2,
            Err(error) => {
                password.zeroize();
                expected_hash.zeroize();
                return Err(error);
            }
        };
        let parsed_hash = match PasswordHash::new(&expected_hash) {
            Ok(parsed_hash) => parsed_hash,
            Err(error) => {
                password.zeroize();
                expected_hash.zeroize();
                return Err(PasswordCryptoError::Parse {
                    error: error.to_string(),
                });
            }
        };

        let result = match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(error) => Err(PasswordCryptoError::Verify {
                error: error.to_string(),
            }),
        };
        password.zeroize();
        expected_hash.zeroize();
        result
    })
    .await;

    match join_result {
        Ok(result) => result,
        Err(error) => Err(PasswordCryptoError::Join {
            error: error.to_string(),
        }),
    }
}

/// Perform the `strong_argon2` operation as implemented by this function.
///
/// # Returns
/// A `Result`, either containing the function output or an error.
fn strong_argon2() -> Result<Argon2<'static>, PasswordCryptoError> {
    let params = match Params::new(64 * 1024, 3, 1, Some(32)) {
        Ok(params) => params,
        Err(error) => {
            return Err(PasswordCryptoError::Params {
                error: error.to_string(),
            });
        }
    };

    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}
