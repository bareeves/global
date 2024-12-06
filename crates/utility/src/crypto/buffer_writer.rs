use std::vec::Vec;
use crate::crypto::hash::Hash;

/// A buffer writer that efficiently writes primitive types and custom data to a vector.
#[derive(Clone)]
pub struct BufferWriter {
    content: Vec<u8>,
}

impl BufferWriter {
    /// Creates a new, empty `BufferWriter`.
    pub fn new() -> Self {
        BufferWriter {
            content: Vec::new(),
        }
    }

    /// Writes a variable-length `u64` to the buffer.
    pub fn put_var_u64(&mut self, i: u64) {
        const U8_LIMIT: u64 = 253;
        const U16_LIMIT: u64 = u16::MAX as u64;
        const U32_LIMIT: u64 = u32::MAX as u64;

        if i < U8_LIMIT {
            self.put_u8(i as u8);
        } else if i <= U16_LIMIT {
            self.put_u8(253);
            self.put_u16(i as u16);
        } else if i <= U32_LIMIT {
            self.put_u8(254);
            self.put_u32(i as u32);
        } else {
            self.put_u8(255);
            self.put_u64(i);
        }
    }

    /// Writes an unsigned 8-bit integer to the buffer.
    pub fn put_u8(&mut self, i: u8) {
        self.content.push(i);
    }

    /// Writes an unsigned 16-bit integer (little-endian) to the buffer.
    pub fn put_u16(&mut self, i: u16) {
        self.content.extend_from_slice(&i.to_le_bytes());
    }

    /// Writes an unsigned 32-bit integer (little-endian) to the buffer.
    pub fn put_u32(&mut self, i: u32) {
        self.content.extend_from_slice(&i.to_le_bytes());
    }

    /// Writes an unsigned 64-bit integer (little-endian) to the buffer.
    pub fn put_u64(&mut self, i: u64) {
        self.content.extend_from_slice(&i.to_le_bytes());
    }

    /// Writes a hash to the buffer by converting it to bytes.
    pub fn put_hash(&mut self, h: &Hash) {
        self.put_bytes(h.as_bytes());
    }

    /// Writes raw bytes to the buffer.
    pub fn put_bytes(&mut self, buf: &[u8]) {
        self.content.extend_from_slice(buf);
    }

    /// Returns a copy of the current buffer content.
    pub fn get_bytes(&self) -> Vec<u8> {
        self.content.clone()
    }
}
