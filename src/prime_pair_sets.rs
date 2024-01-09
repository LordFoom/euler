///The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating them in
/// any order the result will always be prime. For example, taking 7 and  109 , both 7109 and 1097 are prime.
/// The sum of these four primes, 792, represents the lowest sum for a set of four primes with this property.
//
// Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.

pub fn produce_five_prime_set(){
    let four_primes: Vec<u128>= vec![3,7,109,673];

}

pub fn test_prime_concat(primes: &mut &Vec<u128>) {
    for i in 0..4 {
        for j in i+1..5 {
            let first_prime = primes[i];
            let second_prime = primes[i];
        }
    }
    //go through the pairs of primes and concat them
    for first_prime in &primes[..primes.len()-2] {
        for second_prime in &mut  primes[1..primes.len()-1] {
            println!("{},{}", first_prime, second_prime);
            println!("{},{}", second_prime, first_prime);
        }
    }
}

mod test{
    use super::*;

    #[test]
    pub fn test_test_prime_concat() {
        let four_primes: Vec<u128>= vec![3,7,109,673];
        test_prime_concat(&mut &four_primes);

}
}