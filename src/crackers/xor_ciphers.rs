use crate::bitflips::xor::*;
use crate::scorers::english_scorers::*;
use crate::scorers::hamming_distance::hamming_distance;

/// Returns the plaintext encoded using a single-byte XOR cipher. Works by selecting the XOR key
/// that results in the most "english-like" plaintext.
// TODO - Should return &[u8].
pub fn crack_single_byte_xor_cipher(ciphertext: &[u8]) -> String {
    let plaintext = (0u8..255)
        .map(|x| xor(ciphertext, &x))
        .max_by(|x, y| english_score(x).total_cmp(&english_score(y)))
        .expect("we know a maximum will be found");

    return String::from_utf8_lossy(plaintext.as_slice()).to_string();
}

/// Returns the plaintext encoded using a single-byte XOR cipher among a list of possible
/// ciphertexts.
// TODO - Should return Option<&[u8]>.
pub fn detect_and_crack_single_byte_xor_cipher(possible_ciphertexts: &[&[u8]]) -> Option<String> {
    return possible_ciphertexts
        .iter()
        .map(|x| crack_single_byte_xor_cipher(x))
        .max_by(|x, y| english_score(x.as_bytes()).total_cmp(&english_score(y.as_bytes())));
}

/// Finds the key size (of between 2 and 40 bytes) used to encrypt a repeating XOR cipher.
pub fn find_key_size_repeating_xor_cipher(ciphertext: &[u8]) -> usize {
    let candidate_keysizes = 2..41;
    candidate_keysizes
        .min_by(|x, y| average_normalised_hamming_distance(ciphertext, x)
            .total_cmp(&average_normalised_hamming_distance(ciphertext, y)))
        .expect("we know a minimum will be found")
}

/// Returns the average, normalised Hamming distance between four blocks of the provided text.
fn average_normalised_hamming_distance(text: &[u8], block_size: &usize) -> f64 {
    // TODO - See if I can use itertools to do this more elegantly.
    let block_one = &text[0..*block_size];
    let block_two = &text[*block_size..*block_size * 2];
    let block_three = &text[block_size * 2..block_size * 3];
    let block_four = &text[block_size * 3..block_size * 4];

    let total_hamming_distance = hamming_distance(block_one, block_two)
        + hamming_distance(block_one, block_three)
        + hamming_distance(block_one, block_four)
        + hamming_distance(block_two, block_three)
        + hamming_distance(block_three, block_four);

    let number_of_comparisons = 5;

    // The result is normalised (by dividing by the block size) and averaged (by dividing by the
    // number of comparisons performed).
    (total_hamming_distance as f64) / (number_of_comparisons as f64 * *block_size as f64)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::crackers::xor_ciphers::*;

    // Solution to Cryptopals set 01 challenge 03.
    #[test]
    fn can_crack_single_byte_xor_cipher() {
        let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected_plaintext = "Cooking MC's like a pound of bacon";

        let ciphertext_bytes = hex::decode(ciphertext).expect("could not convert hex to bytes");
        let plaintext = crack_single_byte_xor_cipher(&ciphertext_bytes);
        assert_eq!(plaintext, expected_plaintext);
    }

    // Solution to Cryptopals set 01 challenge 04.
    #[test]
    fn can_detect_and_crack_single_byte_xor_cipher() {
        let filename = "./src/crackers/4.txt";
        let file = File::open(filename).expect("could not open file");
        let ciphertexts_bytes_vec = BufReader::new(file)
            .lines()
            .map(|x| hex::decode(x.expect("could not read line"))
                .expect("could not convert hex to bytes"))
            .collect::<Vec<Vec<u8>>>();
        let ciphertexts_bytes = ciphertexts_bytes_vec.iter().map(|x| &x[..]).collect::<Vec<&[u8]>>();
        let expected_plaintext = "Now that the party is jumping\n";

        let plaintext = detect_and_crack_single_byte_xor_cipher(&ciphertexts_bytes)
            .expect("could not find plaintext");
        assert_eq!(plaintext, expected_plaintext);
    }

    // Solution to Cryptopals set 01 challenge 06.
    #[test]
    fn can_detect_and_crack_repeating_key_xor_cipher() {
        let filename = "./src/crackers/6.txt";
        let file = File::open(filename).expect("could not open file");
        let ciphertext = BufReader::new(file)
            .lines()
            .map(|x| x.expect(""))
            .collect::<Vec<String>>()
            .join("");

        let keysize = find_key_size_repeating_xor_cipher(ciphertext.as_bytes());
        println!("{}", keysize);

        // TODO - Finish writing this test.
    }
}