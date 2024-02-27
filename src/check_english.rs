use crate::vingere::vigenere_decrypt;
use std::collections::HashMap;

// Define English letter frequency table
const ENGLISH_LETTER_FREQUENCY: [f64; 26] = [
    0.0817, 0.0149, 0.0278, 0.0425, 0.127, 0.0223, 0.0202, 0.0609, 0.0697, 0.0015, 0.0077, 0.0402,
    0.0241, 0.0675, 0.0751, 0.0193, 0.0009, 0.0599, 0.0633, 0.0906, 0.0276, 0.0098, 0.0236, 0.0015,
    0.0197, 0.0007,
];

// Calculate chi-square statistic
fn chi_square(text: &str) -> f64 {
    let observed_frequencies = letter_frequencies(text);
    let mut chi_square = 0.0;
    for (i, &observed_freq) in observed_frequencies.iter().enumerate() {
        let expected_freq = ENGLISH_LETTER_FREQUENCY[i] * text.len() as f64;
        chi_square += (observed_freq as f64 - expected_freq).powi(2) / expected_freq;
    }
    chi_square
}

// Calculate letter frequencies in text
fn letter_frequencies(text: &str) -> Vec<u32> {
    let mut frequencies = vec![0; 26];
    for c in text.chars().filter(|c| c.is_ascii_alphabetic()) {
        let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
        frequencies[index] += 1;
    }
    frequencies
}

// Score the likelihood of text being English
fn english_score(text: &str) -> f64 {
    1.0 / (1.0 + chi_square(text))
}

// Test the likelihood of resulting strings
pub fn test_english_likelihood(results: &[(String, String)]) -> Vec<(String, f64)> {
    results
        .iter()
        .map(|(text, key)| {
            let decrypted_text = vigenere_decrypt(text.clone(), key.clone());
            (decrypted_text.to_owned(), english_score(&decrypted_text))
        })
        .collect()
}
