use std::cell::BorrowMutError;
use std::collections::{BTreeMap, BTreeSet};
use crate::is_prime;

///The prime 41, can be written as the sum of six consecutive primes:
// 41 = 2 + 3 + 5 + 7 + 11 + 13
//
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
//
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.
//
// Which prime, below one-million, can be written as the sum of the most consecutive primes?
fn consecutive_primes(upper_bound: u64) {
    println!("Firing consecutive primes with upper_bound of {upper_bound}");
    let mut primes = Vec::new();
    generate_primes(upper_bound, &mut primes);
    println!("Generated primes");
    let mut max_prime = 0;
    let mut seq_len = 0;
    for i in 0..primes.len() {
        let mut sum = 0;
        for j in i..primes.len() {
            sum += primes.get(j).unwrap();
            if sum > upper_bound {
                break;
            }
            let curr_len = j - i;
            if primes.binary_search(&sum).is_ok()
                && sum > max_prime && curr_len >= seq_len {
                seq_len = curr_len + 1;
                max_prime = sum;
                println!("Found new max prime and sequence, {max_prime} of len {seq_len}");
            }
        }
    }
    println!("The biggest prime that's a sum of consecutive primes is {max_prime}, made up of a sum of {seq_len} primes ");
}

fn generate_primes(upper_bound: u64, primes: &mut Vec<u64>) {
    for i in 2..upper_bound {
        if is_prime(i as u64) {
            // println!("Found prime {i}");
            primes.push(i);
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_consecutive_primes() {
        consecutive_primes(1_000_000);
    }

    #[test]
    fn test_generate_primes() {
        let mut primes = Vec::new();
        generate_primes(1_000_000, &mut primes);
        println!("{primes:?}");
        assert_eq!(25, primes.len());
    }
}
