use crate::is_prime;

fn consecutive_primes(upper_bound: u64){
    //gene
    let mut primes = Vec::new();
    for i in 2..upper_bound {
        generate_primes(upper_bound, &mut primes);
    }

    
}

fn generate_primes(upper_bound: u64, primes: &mut Vec<u64>) -> Vec<u64> {
    let mut primes = Vec::new();
    for i in 2..upper_bound{
        if is_prime(i as u64){
           primes.push(i);
        }
    }
    primes
}