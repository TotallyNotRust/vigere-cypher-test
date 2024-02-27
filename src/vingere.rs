use std::sync::Mutex;

use crate::check_english::{self, test_english_likelihood};

pub fn vigenere_encrypt(text: String, key: String) -> String {
    text.chars()
        .collect::<Vec<char>>()
        .enumerate_alphabetic()
        .map(|(i, c)| {
            if c.is_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let key_offset = key.as_bytes()[i % key.len()] - base;
                let decrypted_char = ((c as u8 - base + (26 + key_offset)) % 26 + base) as char;
                decrypted_char
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_decrypt(text: String, key: String) -> String {
    text.chars()
        .collect::<Vec<char>>()
        .enumerate_alphabetic()
        .map(|(i, c)| {
            if c.is_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let key_offset = key.as_bytes()[i % key.len()] - base;
                let decrypted_char = ((c as u8 - base + (26 - key_offset)) % 26 + base) as char;
                decrypted_char
            } else {
                c
            }
        })
        .collect()
}

fn next_key(key: &[char]) {}

pub fn vigenere_bruteforce(ciphertext: &str) -> Option<String> {
    let max_len = 4;
    let mut results = Mutex::new(Vec::new());
    let mut key = Mutex::new(vec![b'a'; 1]);
    loop {
        let candidate_key: Vec<u8>;

        if let Some(mutex_key) = key.get_mut().ok() {
        
            candidate_key = mutex_key.clone();
            if let Some(x) = increment_key(mutex_key, max_len) {
                *mutex_key = x;
            } else {
                break;
            }
        } else {
            break;
        }
        
        let decrypted_text = vigenere_decrypt(
            ciphertext.to_string(),
            String::from_utf8(candidate_key.clone()).unwrap(),
        );

        if let Some(results) = results.get_mut().ok() {
            results.push(decrypted_text.clone());
        }

    }

    let mut scores = test_english_likelihood(&results.get_mut().unwrap());

    scores.sort_by(|x, y| y.1.total_cmp(&x.1));

    if let Some((x, y)) = scores.first() {
        println!("{}", y);
        return Some(x.to_owned());
    }
    return None;
}

fn increment_key(key: &mut Vec<u8>, max_length: usize) -> Option<Vec<u8>> {
    for i in (0..key.len()).rev() {
        if key[i] < b'z' {
            key[i] += 1;
            return Some(key.clone());
        } else {
            key[i] = b'a';
        }
    }
    if key.len() < max_length {
        println!(" new len {}", key.len() + 1);
        return Some(vec![b'a'; key.len() + 1]);
    }
    None
}

fn print_status(text: String) {
    println!("English score  {:?}", check_english::english_score(&text));
}

trait VectorExt<T> {
    fn enumerate_alphabetic(&self) -> impl Iterator<Item = (usize, char)>;
}

impl VectorExt<char> for Vec<char> {
    fn enumerate_alphabetic(&self) -> impl Iterator<Item = (usize, char)> {
        let mut i: usize = 0;
        self.iter()
            .map(move |x| {
                let r = (i, x.to_owned());
                if x.is_alphabetic() {
                    i += 1
                }
                r
            })
            .into_iter()
    }
}
