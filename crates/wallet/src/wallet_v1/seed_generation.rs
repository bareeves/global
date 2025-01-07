
use utility::hash::hash::Hash;
use utility::hash::hash::HASH_SIZE;
use utility::system::random::generate_random_number;
use utility::system::random::RandomNumberError;
use crate::wallet_v1::seed_wordlist::WORDLIST;
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum GenerateRandomNumbersError {
    #[error("Failed to generate random numbers")]
    GenerateRandomNumberError,
    #[error("RandomNumberError error: {0}")]
    RandomNumberError(#[from] RandomNumberError),
}

pub fn generate_random_numbers(count: usize, min: usize, max: usize) -> Result<Vec<usize>, GenerateRandomNumbersError> {
    let mut tmp_entropy=Vec::new();
    for _ in 0..count{
        let tmp_random=generate_random_number(min, max)?;
        tmp_entropy.push(tmp_random);
    }
    Ok(tmp_entropy)
}


pub fn generate_seed() -> String {

    let entropy=generate_random_numbers(24,0,WORDLIST.len()-1).unwrap();
    let mut random_words_string=String::new();
    
    let mut word_map: HashMap<usize, &str> = HashMap::new();
    for (i, word) in WORDLIST.iter().enumerate() {
        word_map.insert(i, word);
    }

    for index in entropy {
        if let Some(word) = word_map.get(&index) {
            random_words_string.push_str(word);
            random_words_string.push(' '); 
        } else {
            panic!("Invalid index in entropy: {}", index);
        }
    }
    let checksum_number=get_seed_checksum(&random_words_string);
    if let Some(checksum_word) = word_map.get(&checksum_number) {
        random_words_string.push_str(checksum_word);
        //random_words_string.push(' '); 
        return random_words_string
    } else {
        panic!("Invalid checksum_number: {}", checksum_number);   
    }
    panic!("Failed to generate seed");   
}
fn get_seed_checksum(random_words_string:&str) -> usize  {
    let tmp_string=random_words_string.trim().to_string();

    let tmp_hash = Hash::compute_hash(tmp_string.as_bytes());

    let checksum_number=tmp_hash.to_vec()[HASH_SIZE-1] as usize;
    return checksum_number;

}

fn check_seed_checksum(checksum_number: usize,seed: &str) -> bool {
    let seed_without_last_word=remove_last_word(seed);
    let seed_checksum_number=get_seed_checksum(&seed_without_last_word);
    if seed_checksum_number==checksum_number {
        return true;
    } 
    return false;
}

pub fn check_seed(seed: &str) -> bool {
    let words: Vec<&str> = seed.clone().split_whitespace().collect();
    let mut last_word_string =String::new();
    if let Some(last_word) = words.last() {
        last_word_string=last_word.to_string();
        for (index, &item) in WORDLIST.iter().enumerate() {
            if item == *last_word {
                return check_seed_checksum(index,seed)
            }
        }
        return false;
    } 
    return false;
}
fn remove_last_word(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();

    if words.len() <= 1 {
        return String::new(); 
    }

    words[0..words.len() - 1].join(" ") 
}

