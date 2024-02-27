use vingere::vigenere_bruteforce;

mod check_english;
mod vingere;

fn main() {
    println!("{:?}", vigenere_bruteforce("awfomte sbcud"));
}

#[cfg(test)]
mod tests {
    use crate::vingere::{vigenere_decrypt, vigenere_encrypt};

    #[test]
    fn encrypt() {
        let result = vigenere_encrypt("aweomse sacue".to_owned(), "aab".to_owned());

        assert_eq!(result, "awfomte sbcuf");
    }
    #[test]
    fn decrypt() {
        let result = vigenere_decrypt("awfomte sbcuf".to_owned(), "aab".to_owned());

        assert_eq!(result, "aweomse sacue");
    }
}
