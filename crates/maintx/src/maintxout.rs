use thiserror::Error;
use utility::buffer::buffer_reader::BufferReader;
use utility::buffer::buffer_writer::BufferWriter;
use utility::buffer::buffer_reader::BufferReaderError;
use utility::hash::hash::Hash;

pub const MAINTX_OUT_IDENTIFIER_ECDSA: u32 = 1;

// Define errors for the module
#[derive(Debug, Error)]
pub enum MaintxOutError {
    //#[error("Buffer operation failed: {0}")]
    //BufferError(String),
    #[error("Invalid MaintxOut variant")]
    InvalidVariant,
    #[error("Buffer reader error: {0}")]
    BufferReaderError(#[from] BufferReaderError),
}
/*
impl From<std::io::Error> for MaintxOutError {
    fn from(err: std::io::Error) -> Self {
        MaintxOutError::BufferError(err.to_string())
    }
}
*/
// Define a struct for ECDSA MaintxOut
#[derive(Debug, Clone)]
pub struct MaintxOutEcdsa {
    pub value: u64, // in million globals
    pub address: Hash,
}

// Define an enum that contains different MaintxOut variants
#[derive(Debug, Clone)]
pub enum MaintxOut {
    MaintxOutEcdsaVariant(MaintxOutEcdsa),
}

// Implement serialization for MaintxOutEcdsa
impl MaintxOutEcdsa {
    pub fn serialize(&self, writer: &mut BufferWriter) {
        writer.put_u64(self.value);
        writer.put_var_u32(MAINTX_OUT_IDENTIFIER_ECDSA);
        writer.put_hash(self.address.clone());
        writer.put_var_u64(0); // No extra data
    }
}

// Implement methods for MaintxOut
impl MaintxOut {
    pub fn serialize(&self, writer: &mut BufferWriter) {
        match self {
            MaintxOut::MaintxOutEcdsaVariant(data) => data.serialize(writer),
        }
    }

    pub fn is_ecdsa(&self) -> bool {
        matches!(self, MaintxOut::MaintxOutEcdsaVariant(_))
    }

    pub fn matches_address(&self, address: &Hash) -> bool {
        match self {
            MaintxOut::MaintxOutEcdsaVariant(data) => data.address==*address,
        }
    }

    pub fn as_ecdsa(&self) -> Result<&MaintxOutEcdsa, MaintxOutError> {
        match self {
            MaintxOut::MaintxOutEcdsaVariant(data) => Ok(data),
            _ => Err(MaintxOutError::InvalidVariant),
        }
    }

    pub fn get_address(&self) -> Result<Hash, MaintxOutError> {
        self.as_ecdsa().map(|data| data.address.clone())
    }

    pub fn get_value(&self) -> Result<u64, MaintxOutError> {
        self.as_ecdsa().map(|data| data.value)
    }
}

pub fn new_ecdsa_maintxout(value: u64, address: Hash) -> MaintxOut {
    MaintxOut::MaintxOutEcdsaVariant(MaintxOutEcdsa { value, address })
}

pub fn unserialize_maintxout(reader: &mut BufferReader) -> Result<MaintxOut, MaintxOutError> {
    let value = reader.get_u64()?;
    let identifier = reader.get_var_u32()?;

    if identifier == MAINTX_OUT_IDENTIFIER_ECDSA {
        let address = reader.get_hash()?;
        let _extra_data_len = reader.get_var_u64()?;
        Ok(new_ecdsa_maintxout(value, address))
    } else {
        Err(MaintxOutError::InvalidVariant)
    }
}