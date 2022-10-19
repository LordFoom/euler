use crate::{is_prime, make_digit_vec};

/// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
/// is unusual in two ways: (i) each of the three terms are prime, and,
/// (ii) each of the 4-digit numbers are permutations of one another.
/// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes,
/// exhibiting this property, but there is one other 4-digit increasing sequence.
/// What 12-digit number do you form by concatenating the three terms in this sequence?
///
fn prime_permutations() {
    let mut inc_seq = Vec::new();
    'outer: for i in 1000..=9000 {
        if !is_prime(i as u64) {
            continue;
        }
        //get the digts
        let mut digits_vec = Vec::new();
        make_digit_vec(i as usize, &mut digits_vec);
        // println!("Found prime 4 digits {i}");
        let mut sorted_vec = digits_vec.clone();
        sorted_vec.sort();
        inc_seq.push(i);
        //now we increase byh 3330 and check if it's a permutation or if it's a prime
        for j in 1..3 {
            let new_val = i + j * 3330;
            // println!("Checking {new_val}");
            if new_val > 10000 {
                inc_seq.clear();
                break 'outer;
            }
            let mut new_digits_vec = Vec::new();
            make_digit_vec(new_val as usize, &mut new_digits_vec);
            let mut new_digits_vec_sorted = new_digits_vec.clone();
            new_digits_vec_sorted.sort();

            // println!("{new_digits_vec_sorted:?} = {new_digits_vec_sorted:?}?");
            //remove values from the new_digit_vec
            if sorted_vec
                .iter()
                .zip(new_digits_vec_sorted.iter())
                .all(|(a, b)| a == b)
                && is_prime(new_val as u64)
            {
                println!("{sorted_vec:?} is a permutation of {new_digits_vec:?}");

                inc_seq.push(new_val);
            } else {
                // println!("No increasing sequence found, clearing");
                //empty the vec
                inc_seq.clear();
                break;
            }
        }
        if inc_seq.len() == 3 {
            println!("Found the increasing sequence {inc_seq:?}");
            inc_seq.clear();
        }
    } //end for
}

mod test {
    use super::*;

    #[test]
    fn test_prime_permuations() {
        prime_permutations();
    }
}
