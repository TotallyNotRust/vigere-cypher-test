use crate::vingere::vigenere_decrypt;
use std::{collections::HashMap, ops::Index};

// Define English letter frequency table
const ENGLISH_LETTER_FREQUENCY: [f64; 26] = [
    0.64297, 0.11746, 0.21902, 0.33483, 1.00000, 0.17541, 0.15864, 0.47977, 0.54842, 0.01205,
    0.06078, 0.31688, 0.18942, 0.53133, 0.59101, 0.15187, 0.00748, 0.47134, 0.49811, 0.71296,
    0.21713, 0.07700, 0.18580, 0.01181, 0.15541, 0.00583,
];

// Score the likelihood of text being English
pub fn english_score(text: &str) -> f64 {
    if let Some(x) = text
        .to_lowercase()
        .chars()
        .map(|x| {
            if x.is_alphanumeric() {
                ENGLISH_LETTER_FREQUENCY[(x as u8 - b'a') as usize]
            } else {
                0.0
            }
        })
        .reduce(|x, y| x + y)
    {
        x
    } else {
        0.0
    }
}

// Test the likelihood of resulting strings
pub fn test_english_likelihood(results: &[String]) -> Vec<(String, f64)> {
    results
        .iter()
        .map(|text| {
            (text.to_owned(), english_score(&text))
        })
        .collect()
}
