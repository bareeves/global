use num_bigint::BigUint;
use num_traits::{ToPrimitive, FromPrimitive};
use super::hash::Hash;

/// Convert a `Hash` into a `BigUint` (interpreted as a little-endian integer).
pub fn bigint_from_hash(h: &Hash) -> BigUint {
    BigUint::from_bytes_le(h.as_bytes())
}

/// Convert a `BigUint` to its compact representation (similar to Bitcoin's "compact target").
pub fn compact_from_bigint(value: &BigUint) -> u32 {
    let big_lim = BigUint::from(0x00800000u32); // 24-bit boundary
    let mut tvalue = value.clone();
    let mut exponent: u32 = 0;

    // Shift right until the value fits within 24 bits
    while tvalue >= big_lim {
        tvalue >>= 8;
        exponent += 1;
    }

    // Extract the mantissa and shift the value back if necessary
    let mantissa = tvalue.to_u32().unwrap_or(0);
    if mantissa & 0x00800000 != 0 {
        // If mantissa overflows 24 bits, adjust and increment exponent
        tvalue >>= 8;
        exponent += 1;
    }

    // Compact format: [exponent (8 bits)][mantissa (24 bits)]
    (exponent << 24) | (mantissa & 0x007fffff)
}

/// Convert a compact representation back into a `BigUint`.
/// This is the inverse of `compact_from_bigint`.
pub fn bigint_from_compact(compact: u32) -> BigUint {
    let mantissa = compact & 0x007fffff;
    let exponent = (compact >> 24) as u32;

    let mut value = BigUint::from(mantissa);
    if exponent > 3 {
        value <<= 8 * (exponent - 3);
    } else {
        value >>= 8 * (3 - exponent);
    }
    value
}

/// Convert a 64-bit integer to a `BigUint`.
pub fn bigint_from_u64(value: u64) -> BigUint {
    BigUint::from(value)
}
