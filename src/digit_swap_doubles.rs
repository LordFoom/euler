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

mod test {
    use super::*;

    #[test]
    pub fn test_find_number_where_moving_rightmost_digit_doubles() {
        find_number_where_moving_rightmost_digit_doubles();
    }
}
