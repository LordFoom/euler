use std::collections::HashMap;
use crate::xor_decryption_2::extract_file_bytes;

//let's try some frequency analysis
pub fn decrypt_file() {
    //we load the file
    let encrypted_bytes = extract_file_bytes().unwrap();
   //we build 3 lists
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    let mut third_list = Vec::new();

    for (i, byte) in encrypted_bytes.iter().enumerate() {
        match i%3 {
            0 => first_list.push(*byte),
            1 => second_list.push(*byte),
            2 => third_list.push(*byte),
            err => panic!("How can mod 3 produce {} ", err),
        }
    }

    let first_max = get_most_common(first_list);
    let second_max = get_most_common(second_list);
    let third_max = get_most_common(third_list);

    let first_char = ' ' as u8 ^ first_max;
    let second_char = ' ' as u8 ^ second_max;
    let third_char = ' ' as u8 ^ third_max;

    println!("Supected key is {}{}{}", first_char as char, second_char as char, third_char as char);
    let key = vec![first_char, second_char, third_char];

    let mut decrypted_string = String::new();
    let mut sum: u32 = 0;
    for (i, byte) in encrypted_bytes.iter().enumerate() {
        let decrypted_byte = key[i % 3] ^ byte;
        decrypted_string.push(decrypted_byte as char);
        sum += decrypted_byte as u32;
    }

    println!("Here is the string\n{}", decrypted_string);
    println!("Here is the sum: {}", sum);

}

pub fn get_most_common(list: Vec<u8>) -> u8 {
    let mut frequencies : HashMap<u8, u32> = HashMap::new();

    for num in list {
        *frequencies.entry(num).or_default() += 1;
    }

    *frequencies.iter().max_by_key(|(_,v)| *v).map(|(k,_)| k).unwrap()
}

mod test{
    use crate::xor_decryption_frequency_analysis::{decrypt_file, get_most_common};

    #[test]
    pub fn test_get_most_common() {
        let numbers: Vec<u8> = vec![1,2,3,2,3,2,3,4,4,8];

        let most_common = get_most_common(numbers);
        assert_eq!(3, most_common);
    }


    #[test]
    pub fn test_decrypt_file(){
        decrypt_file();
    }

}