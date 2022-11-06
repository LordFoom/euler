use crate::make_digit_vec;

/// It can be seen that the number, 125874, and its double, 251748,
/// contain exactly the same digits, but in a different order.
/// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x,
/// contain the same digits.


fn find_smallest_x_multiple() {
    let mut multiple = 0;
    'outer: for x in 1.. {
        let mut x_digits = Vec::new();
        make_digit_vec(x, &mut x_digits);
        //5 and 6 times anything other than 1 is an extra digit after multiplication
        if x_digits[0] == 1 {
            //first digit must be 1, otherwise 6x will have an extra digit
            for i in 2..=C6 {
                //ensure 2x,3x,4x,5x,6x are all permutations
                let x_i = x * i;
                if !is_permutation(x, x_i) {
                    continue 'outer;
                }
                println!("{x} and {x}*{i}={x_i} are permutations");
            }
            multiple = x;
            break;
        }
    };

    println!("Smallest number x with mutiple 2x,3x,4x,5x,6x values being permutations is {multiple}")
}

fn is_permutation(n1: usize, n2: usize) -> bool {
    let mut digits_1 = Vec::new();
    make_digit_vec(n1, &mut digits_1);

    let mut digits_2 = Vec::new();
    make_digit_vec(n2, &mut digits_2);

    if digits_1.len() != digits_2.len() {
        return false;
    }

    digits_1.sort();
    digits_2.sort();
    return digits_1.iter()
                   .zip(digits_2.iter())
                   .all(|(a, b)| a == b);
}


mod test {
    use super::*;

    #[test]
    fn test_minimum_permutation() {
        find_smallest_x_multiple();
    }
}