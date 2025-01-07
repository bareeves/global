//extern crate secp256k1;
//extern crate rand;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, ecdsa::Signature};
use thiserror::Error;
use crate::ecdsa::key_derivation_v1::KeyDerivationError;
use crate::ecdsa::key_derivation_v1::ExtendedSecretKey;
use crate::ecdsa::key_derivation_v1::derive_child_extended_secret_key;

use crate::hash::hash;
use crate::hash::hash::Hash;


/// Custom error type for keypair and signature operations.
#[derive(Debug, Error)]
pub enum EcdsaKeySetError {
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
    #[error("KeyDerivationError error: {0}")]
    KeyDerivationError(#[from] KeyDerivationError),
}

/// Struct representing a keypair with its corresponding address.
/*
#[derive(Debug, Clone)]
pub struct EcdsaKeySet {
    secret_key: SecretKey,
    public_key: PublicKey,
    address: Hash,
}
*/
#[derive(Debug, Clone)]
pub struct EcdsaKeySet {
    extended_secret_key: ExtendedSecretKey,
    derivation_index:u32,
    secret_key: SecretKey,
    public_key: PublicKey,
    address: Hash,
}

impl EcdsaKeySet {
    // Getter methods for accessing the SecretKey and PublicKey
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
    pub fn get_secret_key_bytes(&self) -> Vec<u8> {
        self.secret_key.secret_bytes().to_vec().clone()
    }
    pub fn get_public_key_compressed_bytes(&self) -> Vec<u8> {
        self.public_key.serialize().to_vec().clone()
    }
    pub fn get_address(&self)->Hash{
        self.address.clone()
    }
    pub fn get_derivation_index(&self)->u32{
        self.derivation_index
    }
}
/*
/// Generates a keypair from a given secret key byte slice.
pub fn generate_keyset(secret_key_bytes: &[u8]) -> Result<EcdsaKeySet, EcdsaKeySetError> {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(secret_key_bytes)
        .map_err(|e| EcdsaKeySetError::SecretKeyParseError(e.to_string()))?;
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let address = Hash::compute_hash(&pk.serialize().to_vec());
    Ok(EcdsaKeySet {
        secret_key: sk,
        public_key: pk,
        address,
    })
}
*/
//////////////
pub fn derive_child_key_set(master_extended_secret_key: &ExtendedSecretKey, derivation_index: u32,hardened:bool) -> Result<EcdsaKeySet, EcdsaKeySetError> {
    let child_extended_secret_key = derive_child_extended_secret_key(master_extended_secret_key, derivation_index,hardened)?;
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&child_extended_secret_key.secret_key())
        .map_err(|e| EcdsaKeySetError::SecretKeyParseError(e.to_string()))?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let address = Hash::compute_hash(&public_key.serialize().to_vec());
    //public_key_hex: hex::encode(public_key.serialize()),
    Ok(EcdsaKeySet {
        extended_secret_key: child_extended_secret_key,
        derivation_index,
        secret_key,
        public_key,
        address,
    })
}
/////////////

/// Signs a message hash with the provided keypair.
pub fn sign_messagehash(kp: &EcdsaKeySet, message_hash: Hash) -> Result<Vec<u8>, EcdsaKeySetError> {
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(&message_hash.as_bytes())
        .map_err(|e| EcdsaKeySetError::MessageCreationError(e.to_string()))?;
    let signature = secp.sign_ecdsa(&message, kp.secret_key());
    Ok(signature.serialize_compact().to_vec())
}

/// Verifies a signature given the public key, message hash, and compact signature.
pub fn verify_signature(
    public_key_bytes: &[u8],
    message_hash: Hash,
    compact_signature: &[u8],
) -> Result<bool, EcdsaKeySetError> {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_slice(public_key_bytes)
        .map_err(|e| EcdsaKeySetError::PublicKeyParseError(e.to_string()))?;
    let signature = Signature::from_compact(compact_signature)
        .map_err(|e| EcdsaKeySetError::SignatureCreationError(e.to_string()))?;
    let message = Message::from_digest_slice(&message_hash.as_bytes())
        .map_err(|e| EcdsaKeySetError::MessageCreationError(e.to_string()))?;
    Ok(secp.verify_ecdsa(&message, &signature, &public_key).is_ok())
}

