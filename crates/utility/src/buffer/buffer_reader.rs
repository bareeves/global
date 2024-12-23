use crate::hash::hash::Hash;
use std::fmt;
use thiserror::Error;


/// Errors that can occur in `BufferReader`.
#[derive(Debug, Error)]
pub enum BufferReaderError {
    #[error("Value found exceeds u32")]
    ValueFoundExceedsU32,
    #[error("End of buffer reached")]
    EndOfBuffer,
    //#[error("Invalid data encountered: {0}")]
    //InvalidData(String),
}

/// A reader for sequentially extracting data from a byte buffer.
pub struct BufferReader {
    content: Vec<u8>,
    counter: usize,
}

impl BufferReader {


    /// Creates a new `BufferReader` with the provided content.
    pub fn new(content: Vec<u8>) -> Self {
        BufferReader {
            content,
            counter: 0,
        }
    }

    /// Returns the current read position in the buffer.
    pub fn get_counter(&self) -> u64 {
        self.counter as u64
    }

    /// Reads a variable-length `u64` from the buffer.
    pub fn get_var_u64(&mut self) -> Result<u64, BufferReaderError> {
        let i = self.get_u8()?;
        if i < 253 {
            Ok(u64::from(i))
        } else if i == 253 {
            Ok(u64::from(self.get_u16()?))
        } else if i == 254 {
            Ok(u64::from(self.get_u32()?))
        } else {
            Ok(self.get_u64()?)
        }
    }
    /// Reads a variable-length `u64` from the buffer.
    pub fn get_var_u32(&mut self) -> Result<u32, BufferReaderError> {
        let i = self.get_u8()?;
        if i < 253 {
            Ok(u32::from(i))
        } else if i == 253 {
            Ok(u32::from(self.get_u16()?))
        } else if i == 254 {
            Ok(self.get_u32()?)
        } else {
            return Err(BufferReaderError::ValueFoundExceedsU32);
        }
    }    

    /// Reads a `u8` from the buffer.
    pub fn get_u8(&mut self) -> Result<u8, BufferReaderError> {
        self.read_exact(1).map(|buf| buf[0])
    }

    /// Reads a `u16` (little-endian) from the buffer.
    pub fn get_u16(&mut self) -> Result<u16, BufferReaderError> {
        self.read_exact(2)
            .map(|buf| u16::from_le_bytes(buf.try_into().unwrap()))
    }

    /// Reads a `u32` (little-endian) from the buffer.
    pub fn get_u32(&mut self) -> Result<u32, BufferReaderError> {
        self.read_exact(4)
            .map(|buf| u32::from_le_bytes(buf.try_into().unwrap()))
    }

    /// Reads a `u64` (little-endian) from the buffer.
    pub fn get_u64(&mut self) -> Result<u64, BufferReaderError> {
        self.read_exact(8)
            .map(|buf| u64::from_le_bytes(buf.try_into().unwrap()))
    }
    /// Reads a sequence of bytes with a variable u64 before it
    pub fn get_var_bytes(&mut self,) -> Result<Vec<u8>, BufferReaderError> {
        let tmp_length=self.get_var_u32()?;
        self.get_bytes(tmp_length)
    }
    /// Reads a sequence of bytes of the given length from the buffer.
    pub fn get_bytes(&mut self, length: u32) -> Result<Vec<u8>, BufferReaderError> {
        self.read_exact(length as usize)
            .map(|buf| buf.to_vec())
    }

    /// Helper function for reading an exact number of bytes from the buffer.
    fn read_exact(&mut self, length: usize) -> Result<&[u8], BufferReaderError> {
        if self.counter + length > self.content.len() {
            return Err(BufferReaderError::EndOfBuffer);
        }
        let start = self.counter;
        self.counter += length;
        Ok(&self.content[start..self.counter])
    }
    pub fn get_hash(&mut self) -> Result<Hash, BufferReaderError> {
        let slice = self.read_exact((32 as u32).try_into().unwrap())?;
        assert_eq!(slice.len(), 32, "Slice length must be 32");
        Ok(Hash::new(slice.try_into().unwrap()))
    }
}

impl fmt::Debug for BufferReader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BufferReader {{ counter: {}, content length: {} }}",
            self.counter,
            self.content.len()
        )
    }
}
