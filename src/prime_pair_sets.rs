use std::collections::btree_set::BTreeSet;
use anyhow::Result;
use crate::is_prime_cached;

///The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating them in
/// any order the result will always be prime. For example, taking 7 and  109 , both 7109 and 1097 are prime.
/// The sum of these four primes, 792, represents the lowest sum for a set of four primes with this property.
//
// Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.

pub fn produce_five_prime_set(){
    let four_primes: Vec<u64>= vec![3,7,109,673];

    // let mut five_primes = four_primes.iter().map(|x| *x).collect::<Vec<u64>>();
    let mut five_primes = four_primes.clone();
    five_primes.push(2);
    let mut cache = BTreeSet::new();
    four_primes.iter().for_each(|p| { cache.insert(*p); });
    if is_all_concats_prime(&five_primes, &mut cache).unwrap() {
        println!("Found it on the first try by adding 2");
    }

    for test_num in (3..=1000000).step_by(2){
        //only want distinct primes
        if five_primes.contains(&test_num ) {
            continue;
        }
        let mut five_primes = four_primes.clone();
        five_primes.push(test_num);
        if is_prime_cached(test_num, &mut cache){
            if is_all_concats_prime(&five_primes, &mut cache).unwrap(){
                println!("Found it! {:?}", five_primes);
            }
        }
    }
    println!("Sheeet found nothing");

}


pub fn is_all_concats_prime(primes: &Vec<u64>, cache: &mut BTreeSet<u64>) -> Result<bool> {
    let mut prime = true;
    'outer: for i in 0..primes.len()-1 {
        for j in i+1..primes.len() {
            let first_prime = primes[i];
            let second_prime = primes[j];

            let first_concat_prime = format!("{}{}", first_prime, second_prime).parse::<u64>()?;
            let second_concat_prime = format!("{}{}", second_prime, first_prime).parse::<u64>()?;
            // println!("{}", first_concat_prime);
            // println!("{}", second_concat_prime);
            if !(is_prime_cached(first_concat_prime, cache)
                && is_prime_cached(second_concat_prime, cache)){
                prime = false;
                break 'outer
            }
        }
    }

    Ok(prime)
}

// pub fn is_prime(num_to_check: u64, cache: &mut HashMap<u64,bool>) -> bool {
//     if 2 == num_to_check {
//         return true
//     }
//     if cache.contains_key(&num_to_check) {
//         return true
//     }
//
//     let bound = (num_to_check as f64).sqrt() as u64;
//     let mut prime = true;
//     for i in (3..=bound).step_by(2) {
//         if num_to_check%i==0{
//             println!("{} is NOT prime, divisible by {}", num_to_check, i);
//             prime = false;
//             break;
//         }
//     };
//     if prime  {
//         cache.insert(num_to_check, true);
//     }
//     prime
// }

mod test{
    use super::*;

    #[test]
    pub fn test_test_prime_concat() {
        let four_primes: Vec<u64>= vec![3,7,109,673];
        let mut cache = BTreeSet::new();
        assert!(is_all_concats_prime(&mut &four_primes, &mut cache).unwrap());

    }

    #[test]
    pub fn test_produce_five_prime_set(){
        produce_five_prime_set()
    }
}