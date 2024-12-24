use thiserror::Error;
use utility::hash::hash::{self, Hash};
use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReader;
use utility::buffer::buffer_reader::BufferReaderError;
use utility::ecdsa::ecdsa::verify_signature;
use crate::maintx_out::maintx_out::{MaintxOut, new_ecdsa_maintx_out, unserialize_maintx_out};
use crate::maintx_in::maintx_in::{MaintxIn, new_mainblockrewardtxin, unserialize_maintx_in};

use crate::maintx_in::maintx_in::MaintxInError;
use crate::maintx_out::maintx_out::MaintxOutError;

#[derive(Debug, Clone)]
pub struct Maintx {
    pub version: u32,
    pub vin: Vec<MaintxIn>,
    pub vout: Vec<MaintxOut>,
}

#[derive(Debug, Error)]
pub enum MaintxError {
    //#[error("Failed to read transaction data: {0}")]
    //ReadError(#[from] std::io::Error),

    //#[error("Failed to deserialize transaction: {0}")]
    //DeserializeError(String),

    //#[error("Failed to unserialize transaction - BufferReaderError error: {0}")]
    //UnserializeError(#[from] BufferReaderError),
    #[error("Buffer reader error: {0}")]
    BufferReaderError(#[from] BufferReaderError),

    #[error("MaintxIn error: {0}")]
    MaintxInError(#[from] MaintxInError),
    #[error("MaintxOut error: {0}")]
    MaintxOutError(#[from] MaintxOutError),

}

impl Maintx {
    pub fn verify_signatures(&self) -> bool {
        let hash = self.compute_hash();
        for input in &self.vin {
            if input.is_ecdsa() && !input.check_ecdsa_signature(hash.clone()) {
                return false;
            }
        }
        true
    }

    pub fn compute_hash(&self) -> Hash {
        let mut buffer = BufferWriter::new();
        self.serialize_with_buffer_writer(&mut buffer, false);
        let content = buffer.get_bytes();
        Hash::compute_hash(&content)
    }

    pub fn serialize_with_buffer_writer(&self, buffer: &mut BufferWriter, signing: bool) {
        buffer.put_var_u32(self.version);
        buffer.put_var_u64(self.vin.len() as u64);
        for input in &self.vin {
            input.serialize(buffer, signing);
        }
        buffer.put_var_u64(self.vout.len() as u64);
        for output in &self.vout {
            output.serialize(buffer);
        }
    }

    pub fn serialize_transaction(&self) -> Vec<u8> {
        let mut buffer = BufferWriter::new();
        self.serialize_with_buffer_writer(&mut buffer, true);
        buffer.get_bytes()
    }

    pub fn get_serialization_size(&self) -> usize {
        self.serialize_transaction().len()
    }

    pub fn unserialize_maintx(raw_bytes: Vec<u8>) -> Result<Self, MaintxError> {
        let mut reader = BufferReader::new(raw_bytes);
        let version = reader.get_var_u32()?;

        let vin_len = reader.get_var_u64()?;
        let mut vin = Vec::new();
        for _ in 0..vin_len {
            vin.push(unserialize_maintx_in(&mut reader)?);
        }

        let vout_len = reader.get_var_u64()?;
        let mut vout = Vec::new();
        for _ in 0..vout_len {
            vout.push(unserialize_maintx_out(&mut reader)?);
        }

        Ok(Maintx { version, vin, vout })
    }
}

pub fn new_reward_transaction(mainblock_height: u32, value: u64, fee: u64, pubkey_hash: Hash) -> Maintx {
    Maintx {
        version: 1,
        vin: vec![new_mainblockrewardtxin(mainblock_height)],
        vout: vec![new_ecdsa_maintx_out(value + fee, pubkey_hash)],
    }
}