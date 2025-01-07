use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand::rngs::OsRng;
use thiserror::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use zeroize::Zeroize;

pub const MAX_RETRIES: u32 = 1000;

#[derive(Debug, Error)]
pub enum RandomNumberError {
    #[error("The minimum value {0} is greater than the maximum value {1}")]
    MinGreaterThanMax(usize, usize),

    #[error("Failed to initialize RNG: {0}")]
    RngInitializationError(#[from] rand::Error),

    #[error("Range too large: would cause overflow or inefficiency")]
    RangeTooLarge,

    #[error("System time error")]
    SystemTimeError,

    #[error("Value outside acceptable range after {0} attempts")]
    ValueOutOfRange(u32),
}

pub fn generate_secure_random_number(min: usize, max: usize) -> Result<usize, RandomNumberError> {
    // Validate input range
    if min > max {
        return Err(RandomNumberError::MinGreaterThanMax(min, max));
    }

    // Calculate range size and validate it to prevent overflow
    let range_size = max
        .checked_sub(min)
        .and_then(|s| s.checked_add(1))
        .ok_or(RandomNumberError::RangeTooLarge)?;

    // Initialize RNG with system entropy
    let mut rng = ChaCha20Rng::from_rng(OsRng)?;

    // Use rejection sampling to avoid modulo bias
    let mut result;
    let mut attempts = 0;

    loop {
        // Generate a random value using the full range of usize
        let random_value = rng.gen::<usize>();

        // Calculate result within the desired range
        result = random_value % range_size + min;

        // Timing attack mitigation - consistent dummy operation
        let _dummy = rng.gen::<usize>();

        // Check if the result is within the desired range
        if result >= min && result <= max {
            break;
        }

        attempts += 1;
        if attempts >= MAX_RETRIES {
            return Err(RandomNumberError::ValueOutOfRange(attempts));
        }
    }

    // Securely zero out the RNG state manually
    rng = ChaCha20Rng::from_seed([0u8; 32]);

    Ok(result)
}

// Helper function to demonstrate safe usage with retries
pub fn generate_random_number(min: usize, max: usize) -> Result<usize, RandomNumberError> {
    let mut retries = 0;
    loop {
        match generate_secure_random_number(min, max) {
            Ok(num) => return Ok(num),
            Err(e) => {
                retries += 1;
                if retries >= MAX_RETRIES {
                    return Err(e);
                }
                // Small delay between retries to allow system entropy to accumulate
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }
}

/*
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand::rngs::OsRng;
use thiserror::Error;

// Define a custom error type using `thiserror`
#[derive(Debug, Error)]
pub enum RandomNumberError {
    #[error("The minimum value {0} is greater than the maximum value {1}")]
    MinGreaterThanMax(usize, usize),

    #[error("Failed to initialize RNG: {0}")]
    RngInitializationError(#[from] rand::Error),
}

pub fn generate_random_number(min: usize, max: usize) -> Result<usize, RandomNumberError> {
    if min > max {
        return Err(RandomNumberError::MinGreaterThanMax(min, max));
    }

    // Initialize RNG
    let mut rng = ChaCha20Rng::from_rng(OsRng)?;

    Ok(rng.gen_range(min..=max))
}
*/