
use super::hash::Hash;
use super::hash::HASH_SIZE;

/// Compute the Merkle root for a blockchain-like structure using in-place memory updates.
pub fn compute_root(hashes: &[Hash]) -> Hash {
        if hashes.is_empty() {
            return Hash::new_empty();
        }

        // Create a mutable vector from the input hashes.
        let mut nodes = hashes.to_vec();
        let mut length = nodes.len();

        while length > 1 {
            // Iterate over pairs of nodes and compute the next level in-place.
            for i in (0..length).step_by(2) {
                let left = &nodes[i];

                let right = if i + 1 < length {
                    &nodes[i + 1]
                } else {
                    left // No semicolon here, so this returns &Hash
                };

                // Compute the branch hash and store it in the position for the next level.
                nodes[i / 2] = compute_branch(left, right);
            }

            // Update the length to reflect the number of nodes in the next level.
            length = (length + 1) / 2;
        }

        // The root hash is now the first node.
        nodes[0].clone()
}

/// Compute a branch hash with strict ordering and double SHA-256.
fn compute_branch(left: &Hash, right: &Hash) -> Hash {
    let mut hash_concat = [0u8; HASH_SIZE * 2];
    hash_concat[..HASH_SIZE].copy_from_slice(left.as_bytes());
    hash_concat[HASH_SIZE..].copy_from_slice(right.as_bytes());
    Hash::compute_hash(&hash_concat)
}
/*
    /// Double SHA-256 hash::hash.
    fn double_hash(data: &[u8]) -> Hash {
        let intermediate = sha2::Sha256::digest(data);
        let final_hash = sha2::Sha256::digest(&intermediate);
        let mut hash = [0u8; HASH_SIZE];
        hash.copy_from_slice(&final_hash);
        Hash::new(hash)
    }
*/
