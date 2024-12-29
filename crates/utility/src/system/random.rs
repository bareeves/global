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
