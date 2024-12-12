extern crate secp256k1;
extern crate rand;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, ecdsa::Signature};
use thiserror::Error;

use super::hash;
use super::hash::Hash;

/// Custom error type for keypair and signature operations.
#[derive(Debug, Error)]
pub enum KeyPairError {
    #[error("Failed to parse secret key from slice: {0}")]
    SecretKeyParseError(String),
    #[error("Failed to parse public key from slice: {0}")]
    PublicKeyParseError(String),
    #[error("Failed to create message from digest: {0}")]
    MessageCreationError(String),
    #[error("Failed to create signature from compact form: {0}")]
    SignatureCreationError(String),
    #[error("ECDSA verification failed")]
    VerificationFailed,
}

/// Struct representing a keypair with its corresponding address.
#[derive(Debug, Clone)]
pub struct KeyPair {
    secret_key: SecretKey,
    public_key: PublicKey,
    address: Hash,
}

impl KeyPair {
    /// Getter for the secret key.
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    /// Getter for the public key.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Retrieves the secret key as a byte vector.
    pub fn get_secret_key_bytes(&self) -> Vec<u8> {
        self.secret_key.secret_bytes().to_vec()
    }

    /// Retrieves the public key in compressed form as a byte vector.
    pub fn get_public_key_compressed_bytes(&self) -> Vec<u8> {
        self.public_key.serialize().to_vec()
    }

    /// Retrieves the address of the keypair.
    pub fn get_address(&self) -> Hash {
        self.address.clone()
    }
}

/// Generates a keypair from a given secret key byte slice.
pub fn generate_keypair(secret_key_bytes: &[u8]) -> Result<KeyPair, KeyPairError> {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(secret_key_bytes)
        .map_err(|e| KeyPairError::SecretKeyParseError(e.to_string()))?;
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let address = Hash::compute_hash(&pk.serialize().to_vec());
    Ok(KeyPair {
        secret_key: sk,
        public_key: pk,
        address,
    })
}

/// Signs a message hash with the provided keypair.
pub fn sign_messagehash(kp: &KeyPair, message_hash: Hash) -> Result<Vec<u8>, KeyPairError> {
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(&message_hash.as_bytes())
        .map_err(|e| KeyPairError::MessageCreationError(e.to_string()))?;
    let signature = secp.sign_ecdsa(&message, kp.secret_key());
    Ok(signature.serialize_compact().to_vec())
}

/// Verifies a signature given the public key, message hash, and compact signature.
pub fn verify_signature(
    public_key_bytes: &[u8],
    message_hash: Hash,
    compact_signature: &[u8],
) -> Result<bool, KeyPairError> {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_slice(public_key_bytes)
        .map_err(|e| KeyPairError::PublicKeyParseError(e.to_string()))?;
    let signature = Signature::from_compact(compact_signature)
        .map_err(|e| KeyPairError::SignatureCreationError(e.to_string()))?;
    let message = Message::from_digest_slice(&message_hash.as_bytes())
        .map_err(|e| KeyPairError::MessageCreationError(e.to_string()))?;
    Ok(secp.verify_ecdsa(&message, &signature, &public_key).is_ok())
}

