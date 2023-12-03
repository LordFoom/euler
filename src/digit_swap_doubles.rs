use crate::{get_digits, vec_to_num};

// fn move_rightmost_digit_to_front(orig_num:usize) ->usize{
//     let mut digits =
// }

fn moving_rightmost_digit_to_front_doubles_value(num: usize) -> bool {
    // println!("turning number into digits");
    let mut digits = get_digits(num);
    // if let Some(second_last) = digits.get(digits.len() - 2) {
    //     if second_last % 2 != 0 {
    //         return false;
    //     }
    // }
    // println!("We got those digits");
    //move the rightmost to the front
    if let Some(rightmost_digit) = digits.pop() {
        // if let Some(first_digit) = digits.get(0) {
        //     if first_digit >= &rightmost_digit {
        //         return false;
        //     }
        // }
        digits.insert(0, rightmost_digit);
    } else {
        println!("Failed to pop rightmost digit");
    };

    let new_num = vec_to_num(&digits);
    println!("old_num={num}, new_num={new_num}");
    let parasitic = new_num == num * 4;
    if parasitic {
        println!("Found parasitic number, original = {num}, parasite={new_num}");
    }
    return parasitic;
}

fn find_number_where_moving_rightmost_digit_doubles() {
    let mut found = false;
    let mut i = 1_000_000_000;
    while !found {
        if moving_rightmost_digit_to_front_doubles_value(i) {
            println!("Success!");
            found = true;
            break;
        }
        i = i + 1;
        // if i > 128_999{
        //     break;
        // }
    }
    if !found {
        println!("We failed");
    }
}

fn move_rightmost_digit_to_front(n: usize) -> usize {
    let mut digits = get_digits(n);
    if let Some(rightmost) = digits.pop() {
        digits.insert(0, rightmost);
    }
    let transposed =  digits
        .into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect::<String>();

    transposed.parse().unwrap()
}

// fn find_n_parasitic_number(n: usize)->usize{
fn find_2_parasitic_number() {
    // Pick a one digit integer k such that k ≥ n, and take the period of the repeating decimal k/(10n−1)
    //pick k
    let k = 2;
    //get repeating digits of 2/19
    //first get 1/19
    let decimal: f64 = 1. / 19.;
    let repeating_digits = repeating_decimal(1, 19);
    let digit_string = repeating_digits.replace("(", "").replace(")", "");
    println!("digit string, from here we get to the parasitic number");
    let repeating_number: usize = digit_string.parse::<usize>().unwrap() * k;
    let double_repeating = repeating_number * 2;
    let transposed_number = move_rightmost_digit_to_front(repeating_number);
    //now we check to see if they are the  same
    if double_repeating == transposed_number {
        println!("{repeating_number} is half of the rightmost digit moved to front: {double_repeating} - it is 2 parasitic");
    } else {
        println!("{repeating_number} is NOT half of the rightmost digit moved to front: {double_repeating} - it is 2 parasitic");
    }
}

fn repeating_decimal(numerator: i32, denominator: i32) -> String {
    let mut result = String::new();
    let mut remainder_map = std::collections::HashMap::new();
    let mut remainder = numerator % denominator;

    while remainder != 0 && !remainder_map.contains_key(&remainder) {
        remainder_map.insert(remainder, result.len());

        remainder *= 10;
        result.push_str(&(remainder / denominator).to_string());
        remainder %= denominator;
    }

    if remainder != 0 {
        let start = remainder_map[&remainder];
        result.insert(start, '(');
        result.push(')');
    }

    result
}

mod test {
    use super::*;

    #[test]
    pub fn test_find_number_where_moving_rightmost_digit_doubles() {
        // find_number_where_moving_rightmost_digit_doubles();
        find_2_parasitic_number();
    }

    #[test]
    pub fn test_repeating_decimal() {
        let one_third = 1. / 3.;
        println!("{}", one_third);
        let decimals = repeating_decimal(1, 2);
        println!("repeating decimals = {decimals}");
    }

    #[test]
    pub fn test_move_rightmost_digit_to_front() {
        let number = 123456789;
        let new_number = move_rightmost_digit_to_front(number);
        assert_eq!(912345678, new_number);

        let number = 6789443;
        let new_number = move_rightmost_digit_to_front(number);
        assert_eq!( 3678944, new_number);
    }

    #[test]
    pub fn test_find_2_parasitic_number(){
        find_2_parasitic_number()
    }
}
