use crate::check_english::test_english_likelihood;

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

pub fn vigenere_bruteforce(ciphertext: &str) -> Option<String> {
    let mut results = Vec::new();
    for key_length in 1..=ciphertext.len() {
        let key = vec![b'a'; key_length]; // Start with a key of all 'a's of appropriate length
        let mut candidate_key = key.clone();
        loop {
            let decrypted_text = vigenere_decrypt(
                ciphertext.to_string(),
                String::from_utf8(candidate_key.clone()).unwrap(),
            );
            results.push((
                decrypted_text.clone(),
                String::from_utf8(candidate_key.clone()).unwrap(),
            ));
            // Increment the candidate key
            if !increment_key(&mut candidate_key) {
                break;
            }
        }
    }

    let mut scores = test_english_likelihood(&results);

    scores.sort_by(|x, y| y.1.total_cmp(&x.1));
    if let Some((x, _)) = scores.first() {
        return Some(x.to_owned());
    }
    return None;
}

fn increment_key(key: &mut Vec<u8>) -> bool {
    for i in (0..key.len()).rev() {
        if key[i] < b'z' {
            key[i] += 1;
            return true;
        } else {
            key[i] = b'a';
        }
    }
    false
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
