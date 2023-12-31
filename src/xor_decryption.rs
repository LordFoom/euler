use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::str::from_utf8;
use anyhow::{Context, Result};
use regex::Regex;

//euler question number 59
///

// Each character on a computer is assigned a unique code and the preferred standard is ASCII (American Standard Code for Information Interchange).
// For example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.
//
// A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with a given value,
// taken from a secret key. The advantage with the XOR function is that using the same encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.
//
// For unbreakable encryption, the key is the same length as the plain text message, and the key is made up of random bytes.
// The user would keep the encrypted message and the encryption key in different locations, and without both "halves", it is impossible to decrypt the message.
//
// Unfortunately, this method is impractical for most users, so the modified method is to use a password as a key.
// If the password is shorter than the message, which is likely, the key is repeated cyclically throughout the message.
// The balance for this method is using a sufficiently long password key for security, but short enough to be memorable.
//
// Your task has been made easy, as the encryption key consists of three lower case characters. Using 0059_cipher.txt (right click and 'Save Link/Target As...'),
// a file containing the encrypted ASCII codes, and the knowledge that the plain text must contain common English words,
// decrypt the message and find the sum of the ASCII values in the original text.

///generate a key from aaa to zzz
/// convert to ascii
/// loop through and decrypt by going over the bytes in the 3 letters over and over
// fn xor_word

pub fn decrypt_file(cipher_file: &str)->Result<String> {
    //read the file
    let encrypted_string = extract_encrypted_file_string(cipher_file)?;

    let mut key = String::new();
    let mut end_key = "zzz".to_string();
    let mut final_unencoded_string = String::new();
    loop {
        let next_key = next_key_in_sequence(&mut key);
        let expanded_key = duplicate_to_length(&key, encrypted_string.len());
        if expanded_key.len() != encrypted_string.len() {
            panic!("You have got a key of len {} and a encrypted string of len {} ",
                   expanded_key.len(),
                   encrypted_string.len());
        }
        // let straight_up_unencrypted = encrypted_string.clone() ^ expanded_key;
        //now we xor with the encrypted string
        let intermediate_bytes = encrypted_string
            .as_bytes()
            // .chars()
            .into_iter()
            .zip(expanded_key
                .as_bytes()
                // .chars()
                .into_iter()
            ).map(|(val, key)|  val ^ key)
            .collect::<Vec<u8>>();

        let unencrypted_str = std::str::from_utf8(&intermediate_bytes)?;
        let dict = load_dict(None)?;
        if is_common_english_words(unencrypted_str, dict)? {
            println!("We have decrypted the file!");
            final_unencoded_string = unencrypted_str.to_string();
            break;
        };
        // if end_key == next_key {
        //     // panic!("We failed to decrypt the file {}", "you done messed up");
        //     break;
        // }

    }
    //we manated to find an encoded string
    if !final_unencoded_string.is_empty() {
        //now we do the asciki thingy
        println!("This is the decrypted code: {}", final_unencoded_string);
    }
    Ok(final_unencoded_string)
}

fn extract_encrypted_file_string(cipher_file: &str) -> Result<String> {
    let mut encrypted_file = File::open(cipher_file)?;
    //this is a string of comma separated numbers like 128, 132,001, etc
    let mut encrypted_string_array = String::new();
    encrypted_file.read_to_string(&mut encrypted_string_array).context("failed to read file to string")?;
    //split by ,
    let vec_ascii_code = encrypted_string_array.split(",")
                                               .map(|ascii_code| ascii_code.parse::<u8>().unwrap() )
                                               .collect::<Vec<u8>>();
    let encrypted_string = from_utf8(vec_ascii_code.as_slice())?;
    Ok(encrypted_string.to_string())
}

fn is_common_english_words(word_string: &str, dict: Vec<String>) -> Result<bool> {
    let words = word_string.split(char::is_whitespace).collect::<Vec<&str>>();
    //get rid of anything not a character....wil hyphens mess me up?
    let regex = Regex::new("[^a-zA-Z]+")?;

    let remaining = words
        .into_iter()
        .map(|word| regex.replace_all(word, ""))//strip out anything not a letter
        .filter(|word |!dict.contains(&word.to_string()))
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    return if remaining.is_empty() {
        println!("FOUND ENGLISH WORDS :D, {:?}", word_string);
        Ok(true)
    } else {
        println!("Found non-dictionary words, {:?}", remaining);
        Ok(false)
    }
}

///Generate the next key from aaa to zzz, duplicating to match the full length of the string
/// Takes in the current key
pub fn next_key_in_sequence(key: &str) -> String{
    if key.is_empty() {
        return String::from("aaa");
    }
    if key.len() != 3 {
       panic!("woah, wrong length current key: {}", key);
    }

    let mut idx:i8 = (key.len() - 1 ) as i8;
    let mut new_key = String::new();
    while idx>=0 {
        let last_char = key.chars().nth(idx as usize).unwrap();
        let next_last_char = get_next_char_or_loop(last_char);
        new_key = next_last_char.to_string() + &new_key;
        if next_last_char == 'a' {//then the last char looped}
            idx -= 1;
        }else {
            //we are done
            new_key = key[0..idx as usize].to_string() + &new_key;
            break;
        }
    }
    new_key
}

pub fn duplicate_to_length(key: &str, num_chars: usize) -> String {
    let whole_count = num_chars/key.len();
    //how many
    let part_count = num_chars%key.len();
    let whole_part = key.repeat(whole_count);
    let partial_part = key[0..part_count].to_string();
    let expanded_key = format!("{}{}", whole_part, partial_part );
    expanded_key
}


///Return next char, eg b if a is passed, n if m is passed in.
/// If z is passed in, return 'a'
pub fn get_next_char_or_loop(c: char) -> char {
    if c == 'z' {
        return 'a'
    }

    let next_char = (c as u8 +1) as char;
    next_char
}
///load the dictionary from /usr/share/dict/words by default and put it in a sorted vector
pub fn load_dict(maybe_file_path: Option<String>)->Result<Vec<String>, io::Error> {
    let dict_path = if let Some(file_path) = maybe_file_path {
        file_path
    } else {
        "/usr/share/dict/words".to_string()
    };

    let file = File::open(dict_path)?;
    let mut dict:Vec<String> = Vec::new();
    io::BufReader::new(file)
        .lines()
        .filter(|maybe_line| maybe_line.is_ok())
        .map(|maybe_line| maybe_line.unwrap())
        .for_each(|line| dict.push(line));
    Ok(dict)
}

///This is some blind copying and pasting to open a file and read it into lines
// pub fn read_lines<P>(file_path: P ) -> io::Result<io::Lines<io::BufReader<File>>>
//     where P: AskRef<Path>, {
//     let file = File::open(dict_path)?;
//     Ok(io::BufReader::new(file).lines())
// }

#[cfg(test)]
mod test{
    use crate::xor_decryption::{duplicate_to_length, extract_encrypted_file_string, get_next_char_or_loop, is_common_english_words, load_dict, next_key_in_sequence};

    #[test]
    pub fn  test_load_dict() {
        let dict = load_dict(None).unwrap();
        assert!(dict.len() > 0 );
    }

    #[test]
    pub fn test_get_next_char_or_loop() {
           let b_char = get_next_char_or_loop('a');
        assert_eq!('b', b_char);
        let n_char = get_next_char_or_loop('m');
        assert_eq!('n', n_char);
        let a_char = get_next_char_or_loop('z');
        assert_eq!('a', a_char);
    }

    #[test]
    pub fn test_next_key_in_sequence(){
        let key = "aaa";

        let next_key = next_key_in_sequence(key);
        assert_eq!("aab", next_key);

        let key = "aaz";
        let next_key = next_key_in_sequence(key);
        assert_eq!("aba", next_key);

        let key = "aba";
        let next_key = next_key_in_sequence(key);
        assert_eq!("abb", next_key);

        let key = "abz";
        let next_key = next_key_in_sequence(key);
        assert_eq!("aca", next_key);

        let key = "azz";
        let next_key = next_key_in_sequence(key);
        assert_eq!("baa", next_key);
    }

    #[test]
    pub fn test_duplicate_to_length(){
        let key = "aza";
        let key_duplicated = duplicate_to_length(key, 14);
        assert_eq!("azaazaazaazaaz", key_duplicated);

        let key = "12345";
        let key_duplicated = duplicate_to_length(key,17);
        assert_eq!("12345123451234512", key_duplicated);
    }

    #[test]
    pub fn test_is_common_english_words(){
        let cew = "hello darkness my old friend!";
        let dict = load_dict(None).unwrap();
        let english_words = is_common_english_words(cew, dict).unwrap();
        assert!(english_words)
    }

    // #[test]
    // pub fn  test_decrypt_file(){
    //
    // }

    #[test]
    pub fn test_extract_encrypted_file_string(){
        let extracted_encrypted_string = extract_encrypted_file_string("./0059_cipher.txt").unwrap();
        println!("extracted_encrypted_string: {}", extracted_encrypted_string);
    }
}
