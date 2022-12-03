use itertools::Itertools;
use num_bigint::BigInt;



fn lychrel_numbers()-> usize {
    let mut lychrel_count = 0;
    for num in 1..10_000 {
        let mut curr_num = BigInt::from(num);
        let mut found_palindrome = false;
        for count in 1..50 {
            let next_num = reverse_and_add(curr_num);
            if is_palindrome( &next_num ) {
                found_palindrome = true;
                // lychrel_count += 1;
                break;
            }
            curr_num = next_num.clone();
        }
        if !found_palindrome {
            lychrel_count += 1;
        }
    }
    lychrel_count
}

fn reverse_and_add(num: BigInt) -> BigInt {
    let rev_num = reverse_number(&num);
    num + rev_num
}

///Reverse a number eg 12345 -> 54321
pub fn reverse_number(num: &BigInt) -> BigInt {
    if num < &BigInt::from(10) {//reversing single digit is just the single digit
        return num.clone();
    }
    let mut num_vec = Vec::new();
    make_digit_vec(num, &mut num_vec);
    num_vec.reverse();
    let rev_num_str = num_vec.iter().map(|digit| digit.to_string()).join("");
    let rev_num = rev_num_str.parse().unwrap();
    rev_num
}

fn make_digit_vec(n: &BigInt, dig_vec: &mut Vec<BigInt>) {
    let big_ten = BigInt::from(10);
    if n >= &big_ten {
        let next_int = n / big_ten;
        make_digit_vec(&next_int, dig_vec);
    }
    dig_vec.push(n % 10);
}

pub fn is_palindrome(num: &BigInt) -> bool {
    let str_num = num.to_string();
    let str_num_reverse = str_num.chars().rev().collect::<String>();
    let is_pal = str_num == str_num_reverse;
    if is_pal {
        println!("Found a palindrome number: {}", str_num)
    }
    is_pal
}
mod test {
    use super::*;

    #[test]
    pub fn test_reverse_number() {
        let one_five = BigInt::from(12345);
        let rev_num = reverse_number(&one_five);
        let b2 = BigInt::from(54321);
        assert_eq!(b2, rev_num);

        let b3 = BigInt::from(60325);
        let rev_num = reverse_number(&b3);
        let b4 = BigInt::from(52306);
        assert_eq!(b4, rev_num);
    }


    #[test]
    pub fn test_lychrel_count(){
        let count = lychrel_numbers();
        println!("We found so many lychrel numbers: {count}")
    }
}