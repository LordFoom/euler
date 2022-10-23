use crate::{is_prime, make_digit_vec};

/// By replacing the 1st digit of the 2-digit number *3, it turns out that
/// six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
///
/// By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the
/// first example having seven primes among the ten generated numbers, yielding the family:
/// 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member
/// of this family, is the smallest prime with this property.
/// Find the smallest prime which, by replacing part of the number (
/// not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

fn prime_digit_replacement() {
    //we make permutations
    //has to end on an odd number
    for i in 100..=999{
        // is_prime()
        //now we have triples and replace add them in at various spots
        for repeater in 0..=9{
            // let num_to_check =

        }
    }
}


mod test {
    use super::*;

    #[test]
    fn test_prime_digit_replacement() {}
}