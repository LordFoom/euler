use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use anyhow::{Context, Result};

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
    let mut encrypted_file  = File::open(cipher_file)?;
    let mut num_chars = String::new();
    encrypted_file.read_to_string(&mut num_chars).context("failed to read file to string")?;
    let mut key = String::new();
    let mut end_key = "zzz".to_string();
    loop {
        let next_key = next_key_in_sequence(&mut key);
        // let expanded key = duplicate_to_length(key, )
        if end_key == next_key {
            break;
        }

    }
    Ok(String::new())
}

///Generate the next key from aaa to zzz, duplicating to match the full length of the string
/// Takes in the current key
pub fn next_key_in_sequence(key: &mut String) -> String{
    if key.is_empty() {
        return String::from("aaa");
    }
    if key.len() != 3 {
       panic!("woah, wrong length current key: {}", key);
    }
    //split into chars
    let last_char = key.chars().nth(2).unwrap();
    let next_char = get_next_char_or_loop(last_char);

    String::from("UNIMPLEMENTED")
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
    use crate::xor_decryption::{get_next_char_or_loop, load_dict};

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
}
