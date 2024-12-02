use std::fmt;
use sha3::{Digest, Sha3_256};
const HASH_SIZE: usize = 32;
#[derive(Clone)] 
pub struct Hash([u8; 32]);
/*
impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({:02X?})", &self.0)
    }
}
*/
impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl Hash {
    /// Create a new `Hash` from a fixed-size array of 32 bytes.
    pub fn new(data: [u8; 32]) -> Hash {
        Hash(data)
    }
    pub fn new_empty()->Hash {
        Hash([0; HASH_SIZE])
    }

    /// Get a reference to the inner byte array.
    pub fn to_vec(&self) -> Vec<u8> {
        let bytes=self.0.to_vec();
        bytes
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    pub fn to_hex_string(&self) -> String {
        self.0.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
    }
    pub fn is_eq(&self, other: &Hash) -> bool {
        // Compare the hash values
        self.0 == other.0
    }
    /*
    pub fn clone(&self) -> Self {
        Hash(self.0)
    }*/
    /// Create a `Hash` from a slice of bytes.
    /// Returns an error if the slice length is not 32.
    pub fn from_bytes(bytes: &[u8]) -> Result<Hash, &'static str> {
        if bytes.len() != 32 {
            return Err("Invalid hash length");
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(bytes);
        Ok(Hash(hash))
    }


}

    /// Compute a double SHA3-256 hash of the input data and return it as a `Hash` instance.
    pub fn compute_hash(data: &[u8]) -> Hash {

        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let sha3_256_hash = hasher.finalize();

        let mut hash = [0u8; 32];
        hash.copy_from_slice(&sha3_256_hash);
        Hash(hash)
    }
    //
    ///////
    //////
    ///
    pub fn compute_root(hashes: &[Hash]) -> Hash {
        let mut nodes: Vec<Hash> = hashes.to_vec();
    
        let mut length = nodes.len();
        while length > 1 {
            let mut i = 0;
            while i < length {
                let left = &nodes[i];
    
                let right = if i + 1 < length {
                    &nodes[i + 1]
                } else {
                    &nodes[i]
                };
    
                nodes[i / 2] = *compute_hash_tree_branch(left, right);
    
                i += 2;
            }
            length = length / 2 + length % 2;
        }
        nodes[0].clone()
    }
    
    fn compute_hash_tree_branch(left: &Hash, right: &Hash) -> Box<Hash> {
        let mut hash_concat: [u8; HASH_SIZE * 2] = [0; HASH_SIZE * 2];
        hash_concat[..HASH_SIZE].copy_from_slice(left.as_bytes());
        hash_concat[HASH_SIZE..].copy_from_slice(right.as_bytes());
        let result = compute_hash(&hash_concat);
        Box::new(result)
    }