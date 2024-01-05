use std::fs::File;
use std::io::Read;
use anyhow::Result;

pub fn decrypt_file() {
    //we load the file
    let bytes = extract_file_bytes().unwrap();

    let mut decoded_string = String::new();
    for first in 'a'..='z' {
        for second in 'a'..='z'{
            for third in 'a'..='z'{
                decoded_string = String::new();
                let key = vec![first, second, third];
                for (i,byte) in bytes.iter().enumerate() {
                    // let key_char = match i%3 {
                    //     2 => first as u8,
                    //     1 => second as u8,
                    //     0 => third as u8,
                    //     _ => panic!("No way hosier jay!"),
                    // };

                    // println!("decoding char: {}", key_char as char);
                    let decoded_char = key[i%3] as u8 ^ byte;
                    if invalid_char(decoded_char) {
                        println!("Breaking as {} is invalid, key={}{}{}",
                                 decoded_char as char, first, second, third);
                        println!("So far we have {}", decoded_string);
                        decoded_string = String::new();
                        break;
                    }
                    decoded_string.push(decoded_char as char);
                }
            }
        }
    }
    println!("Decoded string {}", decoded_string);
   let sum:u32 = decoded_string
       .chars()
       .into_iter()
       .map(|c| c as u32)
       .sum();

    println!("The sum is: {}", sum);
}

//The file is stored as u8 representing bytes
pub fn extract_file_bytes() -> Result<Vec<u8>> {
    let mut file_string = String::new();
    File::open("./0059_cipher.txt")?
        .read_to_string(&mut file_string)?;
    let bytes = file_string
        .split(",")
        .map(|byte| byte.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    Ok(bytes)
}

pub fn invalid_char(test_byte: u8) -> bool{
   !valid_char_ascii(test_byte)
}
///true iff test_num is a space or a lowercase or uppercase letter
/// or .,;()
const ACCEPTABLE_CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz .,:;+?!/[]()1234567890'\"";
pub fn valid_char_ascii(test_byte: u8) -> bool {
    let mut test_char = test_byte as char;
    if let Some(i) = ACCEPTABLE_CHARACTERS.chars().position(|c| c == test_char) {
        return true;
    }
    false
    // 37 != test_byte && 38 != test_byte && //no & or %
    //     ((32 <= test_byte && test_byte<=46) //punctuation and space
    //     || 63  == test_byte//questionmark
    //     || ( 65 <= test_byte && test_byte <= 91 ) || (97<= test_byte && test_byte <= 122) //characters
    // || ( 48<= test_byte && test_byte<=57)) //numbers

}

mod test{
    use crate::xor_decryption_2::{decrypt_file, invalid_char, valid_char_ascii};

    #[test]
    pub fn test_valid_char_ascii() {
        let zlc = 'z' as u8;
        let zuc = 'Z' as u8;
        let alc = 'a' as u8;
        let auc = 'A' as u8;
        let space = ' ' as u8;
        let asterisk = '*' as u8;
        let slash = '/' as u8;

        assert!(valid_char_ascii(zlc));
        assert!(valid_char_ascii(zuc));
        assert!(valid_char_ascii(alc));
        assert!(valid_char_ascii(auc));
        assert!(valid_char_ascii(space));

        assert!(invalid_char(space));
        assert!(invalid_char(slash));
    }

    #[test]
    pub fn test_decrypt_file() {
        decrypt_file()
    }
}