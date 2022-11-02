use crate::consecutive_prime_sum::generate_primes;
use crate::{is_prime, make_digit_vec};
use itertools::Itertools;
use regex::Regex;

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
    // for i in 100..=999{
    //     // is_prime()
    //     //now we have triples and replace add them in at various spots
    //     for repeater in 0..=9{
    //         // let num_to_check =
    //
    //     }
    // }

    //let's assume 3 repeating digits and the number is under 1000000, that is 6 digits
    let mut primes = Vec::new();
    generate_primes(1_000_000, &mut primes);
    // let filtered_primes = primes.iter()
    //                             .filter(|prime| prime > &&( 100000 as u64 ))
    //                             .collect();
    //now let's find primes that match with 3 repeating digits
    //we ...guess, gods help us, the
    let filtered_primes: Vec<u64> = primes
        .clone()
        .into_iter()
        .filter(|prime| prime > &(100_000 as u64))
        .sorted()
        .collect();
    // println!("filtered_primess = {filtered_primes:?}");
    let mut repeat_digit_prime = Vec::new();
    for i in 0..=2 {
        //because we need 8 primes, can't be higher than 2
        let regex_str = format!(r"(\d*){i}(\d*){i}(\d*){i}(\d*)(\d)");
        // println!("{regex_str}");
        let re = Regex::new(&regex_str).unwrap();
        'outer: for prime in filtered_primes.clone() {
            let prim_str = &prime.to_string();
            //found a triple digit
            if re.is_match(prim_str) {
                println!("Found a triple digit! {prim_str}");
                repeat_digit_prime.push(prime);
                // repeat_digi_prime.push(prime);
                let groups = re.captures(prim_str).unwrap();
                let first_part = groups.get(1).map_or("", |g| g.as_str());
                let second_part = groups.get(2).map_or("", |g| g.as_str());
                let third_part = groups.get(3).map_or("", |g| g.as_str());
                let fourth_part = groups.get(4).map_or("", |g| g.as_str());
                let fifth_part = groups.get(5).map_or("", |g| g.as_str());
                println!("Now to replace parts! first ={first_part}, {i}, second={second_part}, {i}, third={third_part}, {i}, fourth={fourth_part}, fifth={fifth_part}");
                // //now we need to keep all the digits but the triple the same
                // //create new numbers and check them for primality
                for j in i + 1..=9 {
                    // let regex_str = format!(r"(\d*){i}(\d*){i}(\d*){i}(\d*)(\d)");
                    let poss_prime_str = format!(
                        "{first_part}{j}{second_part}{j}{third_part}{j}{fourth_part}{fifth_part}"
                    );
                    let poss_prime = poss_prime_str.parse().unwrap();
                    if filtered_primes.binary_search(&poss_prime).is_ok() {
                        // println!("Found a permutation with first = {first_part}, jjj={j}{j}{j}, last = {last_part}");
                        println!("    Found a replacement digit prime: {poss_prime}");
                        repeat_digit_prime.push(poss_prime);
                    }
                    if repeat_digit_prime.len() >= 8 {
                        println!("FOUND 8 prime family! {repeat_digit_prime:?}");
                        break 'outer;
                    }
                }

                repeat_digit_prime.clear();
            }
        }

        println!("Found the following {repeat_digit_prime:?}");
        if repeat_digit_prime.len() >= 8 {
            println!("Found this 8 digit repeating primes: {repeat_digit_prime:?}");
            break;
        }
        repeat_digit_prime.clear();
        // println!("Found the following {repeat_digit_prime:?}");
        // if repeat_digit_prime.len() == 8 {
        //     println!("Found this 8 digit repeating primes: {repeat_digit_prime:?}");
        // }
        // repeat_digit_prime.clear();
    }
    println!("Did we finish?");
}

mod test {
    use super::*;

    #[test]
    fn test_prime_digit_replacement() {
        prime_digit_replacement();
    }

    #[test]
    fn test_regex() {
        let regex_str = format!(r"\d*1{{3}}\d*");

        println!("{regex_str}");
        let regex = Regex::new(&regex_str).unwrap();
        let num1 = 811123.to_string();
        let should_match = regex.is_match(&num1);
        assert!(should_match);

        let num2 = 811213.to_string();
        let should_not_match = regex.is_match(&num2);
        assert!(!should_not_match);
    }
}
