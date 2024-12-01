//extern crate chrono;

use chrono::{Local, TimeZone};

pub fn timestamp_now()->i64 {
    let current_time = Local::now();

    current_time.timestamp()
}
/*
//TODO: For GMT time format
use chrono::{DateTime, TimeZone, Utc};

fn main() {
    let timestamp = 1678730400; // Replace this with your timestamp in seconds
    let dt = Utc.timestamp(timestamp, 0);
    let gmt_string = dt.to_rfc2822(); // You can use other format methods as needed
    println!("{}", gmt_string);
}*/

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand::rngs::OsRng;
use std::error::Error;


//use rand::prelude::*;
//use rand_chacha::ChaCha20Rng;

pub fn generate_random_number(min: usize, max: usize) -> Result<usize, Box<dyn Error>> {
    if min > max {
        return Err("min is greater than max".into());
    }

    let mut rng: ChaCha20Rng;
    rng = ChaCha20Rng::from_rng(OsRng)?;
    Ok(rng.gen_range(min..=max))
}
