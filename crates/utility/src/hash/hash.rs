//use std::fmt;
use thiserror::Error;
use sha3::{Digest, Sha3_256};

pub const HASH_SIZE: usize = 32;

/// Errors related to Hash operations.
#[derive(Error, Debug)]
pub enum HashError {
    /// The input byte slice does not have the expected length of 32.
    #[error("Invalid hash length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
}

/// A wrapper around a 32-byte hash.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hash([u8; HASH_SIZE]);

impl Hash {
    /// Create a new `Hash` from a fixed-size array of 32 bytes.
    pub fn new(data: [u8; HASH_SIZE]) -> Self {
        Self(data)
    }

    /// Create an empty `Hash` initialized to zeroes.
    pub fn new_empty() -> Self {
        Self([0; HASH_SIZE])
    }

    /// Convert the inner byte array to a `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Get a reference to the inner byte array.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert the hash to a hexadecimal string representation.
    pub fn to_hex_string(&self) -> String {
        self.0.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    /// Create a `Hash` from a slice of bytes.
    /// Returns an error if the slice length is not 32.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HashError> {
        if bytes.len() != HASH_SIZE {
            return Err(HashError::InvalidLength {
                expected: HASH_SIZE,
                actual: bytes.len(),
            });
        }
        let mut hash = [0u8; HASH_SIZE];
        hash.copy_from_slice(bytes);
        Ok(Self(hash))
    }

    /// Compute a SHA3-256 hash of the input data and return it as a `Hash`.
    pub fn compute_hash(data: &[u8]) -> Self {
        let hash_bytes = Sha3_256::digest(data);
        let mut hash = [0u8; HASH_SIZE];
        hash.copy_from_slice(&hash_bytes);
        Self(hash)
    }
}
