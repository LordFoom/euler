use std::collections::btree_set::BTreeSet;
use std::collections::HashMap;
use anyhow::Result;
use crate::{is_prime, is_prime_cached};

pub fn calc_prime_list(list_size: usize) -> Result<()> {
    //let us get all primes below 10000
    let primes = generate_primes(100000)?;

    let mut cache = BTreeSet::new();

    let mut candidates:HashMap<u64, Vec<u64>> = HashMap::new();

    //for each prime, find all the other primes that make concat primes
    'outer: for i in 0..primes.len()-2{
        for j in i+1..primes.len()-1{
            let first_prime:u64 = *primes.get(i).unwrap();
            let second_prime:u64 = *primes.get(j).unwrap();

            let first_concat_prime = format!("{}{}", first_prime, second_prime).parse::<u64>()?;
            let second_concat_prime = format!("{}{}", second_prime, first_prime).parse::<u64>()?;

            if is_prime_cached(first_concat_prime, &mut cache)
                && is_prime_cached(second_concat_prime, &mut cache) {
                //we add to  our list
                let prime_vec = candidates
                    .entry(first_prime)
                    .or_insert(Vec::new());
                //now we see if the others in the vec are also concat primes with the new prime
                let mut all_good = true;
                for prior_prime in prime_vec.clone() {
                    let prior_concat_one = format!("{}{}", second_prime, prior_prime).parse::<u64>()?;
                    let prior_concat_two = format!("{}{}", prior_prime, second_prime).parse::<u64>()?;
                    //not a prime pair, we do not bother to add
                    if !(is_prime_cached(prior_concat_one, &mut cache)
                        && is_prime_cached(prior_concat_two, &mut cache)){
                        all_good = false;
                        break;
                    }
                }

                if all_good {
                    prime_vec.push(second_prime);
                    let matching_primes = prime_vec.len();
                    //TODO first one to hit lenght of five has to be the  one because we start with low nubmers? is that correct? feels correct
                    if matching_primes == 4 {
                        println!("Here are 5 primes: {} with {:?}", first_prime, prime_vec);
                        let mut sum = prime_vec.iter().sum::<u64>() + first_prime;
                        println!("Sum is: {}", sum);
                        break 'outer;
                    }
                }

            }
        }
    }
    println!("What the woah?");
    println!("{:?}", candidates);
    Ok(())
}

pub fn generate_primes(boundary:usize)->Result<Vec<u64>> {
   let mut primes:Vec<u64> = Vec::new();

    primes.push(2);
    for n in  (3..=boundary).step_by(2) {
       if is_prime(n as u64) {
          primes.push(n as u64)
        }
    }

    Ok(primes)
}

mod test{
    use super::*;

    #[test]
    pub fn test_generate_primes(){
        let primes = generate_primes(10).unwrap();
        assert_eq!(primes, vec![2,3,5,7]);
    }

    #[test]
    pub fn test_get_prime_list() {
        calc_prime_list(100000).unwrap();
    }
}