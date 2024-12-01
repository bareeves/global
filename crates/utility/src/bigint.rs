//extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::ToPrimitive; // Import ToPrimitive for to_u32() method
use crate::hashing::Hash;

pub fn bigint_from_hash(h: Hash) -> BigUint {
    let mut buf = h.as_bytes();
    BigUint::from_bytes_le(buf)
    
}

pub fn compact_from_bigint(value: &BigUint) -> u32 {
    //let big_two = BigUint::from(2u32);
    let big_lim = BigUint::from(16777216u32);
    let mut exponent: u32 = 0;
    let mut tvalue = value.clone();

    loop {
        if let Some(mantissa) = tvalue.to_u32() {
            let compact = (exponent << 24) | mantissa;
            return compact;
        } else if tvalue < big_lim.clone() {
            let mantissa = tvalue.clone().to_u32().unwrap();
            let compact = (exponent << 24) | mantissa;
            return compact;
        } else {
            tvalue >>= 1;
            exponent += 1;
        }
    }
}

pub fn bigint_from_compact(compact: u32) -> BigUint {
    let mantissa = compact & 0x00ffffff;
    let exponent = compact >> 24;
    let mut value = BigUint::from(mantissa);
    value <<= exponent;
    value
}

pub fn bigint_from_i64(value: u64) -> BigUint {
    BigUint::from(value)
}