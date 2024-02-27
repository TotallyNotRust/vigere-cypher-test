use std::time::Instant;

use vingere::vigenere_bruteforce;

use crate::vingere::vigenere_encrypt;

mod check_english;
mod vingere;

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::{
        check_english::english_score,
        vingere::{vigenere_bruteforce, vigenere_decrypt, vigenere_encrypt},
    };

    #[test]
    fn encrypt() {
        let result = vigenere_encrypt("aweomse sacue".to_owned(), "aab".to_owned());

        assert_eq!(result, "awfomte sbcuf");
    }
    #[test]
    fn decrypt() {
        let result = vigenere_decrypt("o jtfm acbv gschscqs".to_owned(), "oop".to_owned());

        println!("English score: {}", english_score(&result));
        assert_eq!(result, "a very long sentence");
    }
    #[test]
    fn bruteforce() {
        let original_string = "this is an english text that is supposed to be tested by my brute force decryption";
        let x = vigenere_encrypt(
                original_string
                .to_owned(),
            "hans".to_owned(),
        );
        let result = vigenere_bruteforce(&x);

        if let Some(result_unpacked) = result {
            assert_eq!(original_string, result_unpacked);
        }
        else {
            panic!("Could not bruteforce");
        }
    }
}
