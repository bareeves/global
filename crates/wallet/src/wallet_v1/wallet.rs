use std::error::Error;
use thiserror::Error;
use utility::ecdsa;
use utility::hash::hash::{self, Hash};
use utility::utility::generate_random_number;
use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReader;
use utility::bytesfile;
use std::fs;
use maintx::maintx::Maintx;
use crate::resource::{self, Resource};

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Key pair vector is empty")]
    EmptyEcdsaKeyPair,

    #[error("Invalid index: {0}")]
    InvalidIndex(usize),

    #[error("ECDSA error: {0}")]
    EcdsaError(String),

    #[error("Failed to generate random number: {0}")]
    RandomNumberError(String),
}

#[derive(Debug, Clone)]
pub struct WalletInner {
    version: u64,
    vkp: Vec<ecdsa::EcdsaKeyPair>,
    last_known_height: usize,
    vresource: Vec<Resource>,
}

impl WalletInner {
    pub fn new() -> Self {
        WalletInner {
            version: 1,
            vkp: Vec::new(),
            last_known_height: 0,
            vresource: Vec::new(),
        }
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    pub fn get_last_known_height(&self) -> usize {
        self.last_known_height
    }

    pub fn set_last_known_height(&mut self, height: usize) {
        self.last_known_height = height;
    }

    pub fn get_secret_key_bytes(&self, index: usize) -> Result<Vec<u8>, WalletError> {
        self.vkp
            .get(index)
            .map(|kp| kp.get_secret_key_bytes().clone())
            .ok_or(WalletError::InvalidIndex(index))
    }

    pub fn get_public_key_compressed_bytes(&self, index: usize) -> Result<Vec<u8>, WalletError> {
        self.vkp
            .get(index)
            .map(|kp| kp.get_public_key_compressed_bytes().clone())
            .ok_or(WalletError::InvalidIndex(index))
    }

    pub fn get_address(&self, index: usize) -> Result<Hash, WalletError> {
        self.vkp
            .get(index)
            .map(|kp| kp.get_address())
            .ok_or(WalletError::InvalidIndex(index))
    }

    pub fn get_addresses(&self) -> Vec<Hash> {
        self.vkp.iter().map(|kp| kp.get_address()).collect()
    }

    pub fn get_keypairs_count(&self) -> usize {
        self.vkp.len()
    }

    pub fn get_keypair(&self, index: usize) -> Result<ecdsa::EcdsaKeyPair, WalletError> {
        self.vkp
            .get(index)
            .cloned()
            .ok_or(WalletError::InvalidIndex(index))
    }

    pub fn get_random_keypair(&self) -> Result<ecdsa::EcdsaKeyPair, WalletError> {
        let count = self.vkp.len();
        if count == 0 {
            return Err(WalletError::EmptyEcdsaKeyPair);
        }
        let index = generate_random_number(0, count - 1).map_err(|e| WalletError::RandomNumberError(e.to_string()))?;
        Ok(self.vkp[index].clone())
    }

    pub fn generate_keypair(&mut self) -> Result<(), WalletError> {
        if self.vkp.is_empty() {
            return Err(WalletError::EmptyEcdsaKeyPair);
        }
        let last_secret_key = self.vkp.last().unwrap().get_secret_key_bytes();
        let last_secret_key_hash = hash::hash::compute_hash(&last_secret_key);
        let new_kp = ecdsa::generate_keypair(&last_secret_key_hash.to_vec())
            .map_err(|e| WalletError::EcdsaError(e.to_string()))?;
        self.vkp.push(new_kp);
        Ok(())
    }

    pub fn update_resources(&mut self, tmpmaintx: Maintx) -> Result<(), WalletError> {
        let addresses = self.get_addresses();

        for (i, vout) in tmpmaintx.vout.iter().enumerate() {
            for (j, address) in addresses.iter().enumerate() {
                if vout.is_ecdsamaintx_out_address(address) {
                    let hash = tmpmaintx.compute_hash();
                    self.add_unspent_resource(hash, i as u32, vout.get_value()?, j);
                }
            }
        }

        for vin in &tmpmaintx.vin {
            if vin.is_ecdsamaintx_in() {
                self.update_resource_to_spent(vin.get_hash()?, vin.get_index()?);
            }
        }

        Ok(())
    }

    pub fn add_unspent_resource(&mut self, h: Hash, tmpindex: u32, value: u64, key_index: usize) {
        if self.vresource.iter().any(|r| r.hash.is_eq(&h) && r.index == tmpindex) {
            return;
        }
        let new_resource = resource::new_unspent_resource(h, tmpindex, value, key_index);
        self.vresource.push(new_resource);
    }

    pub fn update_resource_to_spent(&mut self, h: Hash, tmpindex: u32) {
        if let Some(resource) = self.vresource.iter_mut().find(|r| r.hash.is_eq(&h) && r.index == tmpindex) {
            resource.update_resource_to_spent();
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.vresource
            .iter()
            .filter(|r| r.is_unspent_resource())
            .map(|r| r.value)
            .sum()
    }

    pub async fn save_addressesfile(&self, path: String) -> Result<(), WalletError> {
        let mut bw = BufferWriter::new();
        bw.put_var_u64(self.vkp.len() as u64);

        for kp in &self.vkp {
            let tmp_addr = hash::hash::compute_hash(&kp.get_public_key_compressed_bytes());
            bw.put_hash(tmp_addr);
        }

        for i in 0.. {
            let file_path = format!("MiningFiles/AddressesFile{}", i);
            if fs::metadata(&file_path).is_err() {
                bytesfile::save_bytes_to_file(&bw.get_bytes(), &file_path).await.map_err(|e| WalletError::EcdsaError(e.to_string()))?;
                break;
            }
        }
        Ok(())
    }
}

pub fn new_walletinner(wallet_seed: String) -> Result<WalletInner, WalletError> {
    let mut new_walletinner = WalletInner::new();

    let mut tmphash = hash::hash::compute_hash(wallet_seed.as_bytes());
    let stretching_factor: u64 = 1000;
    for i in 0..stretching_factor {
        if ((i + 1) * 10) % stretching_factor == 0 {
            tmphash = hash::hash::compute_hash(tmphash.as_bytes());
        }
    }

    let initial_kp = ecdsa::generate_keypair(tmphash.as_bytes()).map_err(|e| WalletError::EcdsaError(e.to_string()))?;
    new_walletinner.vkp.push(initial_kp);

    for _ in 0..3 {
        new_walletinner.generate_keypair()?;
    }

    Ok(new_walletinner)
}
