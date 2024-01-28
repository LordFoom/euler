use std::collections::btree_set::BTreeSet;
use anyhow::Result;
use crate::{is_prime_cached};

pub fn find_prime_pair_set()->Result< Vec<u64> > {

    let mut cache = BTreeSet::new();

    //arbitrary boundary
    let primes:Vec<u64> = generate_primes(10000, &mut cache)?;
    let mut sums: Vec<u64> = Vec::new();
    for i in 0..primes.len()-2 {
        let mut candidates:Vec<u64> = Vec::new();
        let small_prime = primes.get(i).unwrap();
        for j in i+1..primes.len()-1{
            let big_prime = primes.get(j).unwrap();
            if is_prime_pair(small_prime, big_prime, &mut cache)?{
                candidates.push(*big_prime);
            }
        }
        // println!("Here are the candidates for {}, {:?}", small_prime, candidates);
        // println!("Checking {} for sets", small_prime);
        //now we have all the pairs that work with i, let us see if they work for each other
        // sums.push(i as u64);
        for first_test_index in 0..primes.len()-4 {
            let first_test_prime = primes.get(first_test_index).unwrap();
            for second_test_index in first_test_index+1..primes.len()-3{
                let second_test_prime = primes.get(second_test_index).unwrap();
                if !is_prime_pair(first_test_prime, second_test_prime, &mut cache)?{
                   continue;
                }
                for third_test_index in second_test_index+1..primes.len()-2{
                    let third_test_prime = primes.get(third_test_index).unwrap();
                    if !is_prime_pair(first_test_prime, third_test_prime, &mut cache)?{
                        // println!("Failed on the third check, {}, {}, {}", small_prime, first_test_prime, third_test_prime);
                        continue;
                    }
                    if !is_prime_pair(second_test_prime, third_test_prime, &mut cache)?{
                        // println!("Failed on the third check, {}, {}, {}", small_prime, second_test_prime, third_test_prime);
                        continue;
                    }

                    for fourth_test_index in third_test_index+1..primes.len()-3{
                        let fourth_test_prime = primes.get(fourth_test_index).unwrap();

                        if !is_prime_pair(first_test_prime, fourth_test_prime, &mut cache)?{
                            // println!("Failed on the fourth check, {}, {}, {}", small_prime, first_test_prime, fourth_test_prime);
                            continue;
                        }
                        if !is_prime_pair(second_test_prime, fourth_test_prime, &mut cache)?{
                            // println!("Failed on the fourth check, {}, {}, {}", small_prime, second_test_prime, fourth_test_prime);
                            continue;
                        }
                        if !is_prime_pair(third_test_prime, fourth_test_prime, &mut cache)?{
                            continue;
                        }
                        let prime_string = println!("Found a set of 5 primes that are prime pairs with each other: {},{},{},{},{}",
                                                    small_prime, first_test_prime, second_test_prime, third_test_prime, fourth_test_prime);
                        let sum = small_prime+first_test_prime+second_test_prime+third_test_prime+fourth_test_prime;
                        sums.push(sum);
                    }
                }
            }
        }


    }

    Ok(sums)
    //now let us check the damn
    // for i in candidates
}

pub fn is_prime_pair(small_prime: &u64, big_prime: &u64, cache: &mut BTreeSet<u64>) -> Result<bool> {
    //this can be done better by checking bounds i guess?
    let first_prime_concat = format!("{}{}", small_prime, big_prime).parse::<u64>()?;
    let second_prime_concat = format!("{}{}", big_prime, small_prime).parse::<u64>()?;
    if is_prime_cached(first_prime_concat, cache) &&
        is_prime_cached(second_prime_concat, cache) {
        return Ok(true)
    }
    return Ok(false);
}

pub fn generate_primes(boundary:usize, cache: &mut BTreeSet<u64>)->Result<Vec<u64>> {
    let mut primes:Vec<u64> = Vec::new();

    //we can exclude 2 because concat will make even numbers
    // primes.push(2);
    for n in  (3..=boundary).step_by(2) {
        if n == 5 {
            continue;//same as 2, concat will create a divide by 5 number
        }
        if is_prime_cached(n as u64, cache) {
            primes.push(n as u64)
        }
    }

    Ok(primes)
}

mod test{
    use crate::prime_pairs_3::find_prime_pair_set;

    #[test]
    pub fn test_first_prime_pair_set(){
        find_prime_pair_set().unwrap();
    }
}
