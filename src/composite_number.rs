fn is_prime(num_to_check: u64) -> bool {
    let is_prime = !(2..=((num_to_check as f64).sqrt() as u64))
        .any(|i| num_to_check % i == 0);
    // if is_prime{
    //     println!("Found prime number {num_to_check}")
    // }
    // println!("Is {} prime? {}", num_to_check, factor_found);
    is_prime
}

fn composite_number() {
    let mut smallest_defiant_composite = 0;
    for i in (35..).step_by(2) {
        if !is_prime(i) {//non prime
            let mut found_prime = false;
            //now we subtract squares  and test for primes
            for j in 1..((i as f64).sqrt() as i64) {
                let remainder: i64 = i as i64 - 2 * j.pow(2);
                if remainder <= 0 {
                    break;
                }
                if remainder > 0 && is_prime(remainder as u64) {
                    found_prime = true;
                    break;
                }
            }
            if !found_prime {
                smallest_defiant_composite = i;
                break;
            }
        }
    }
    println!("This number breaks the conjecture: {smallest_defiant_composite}");
}

mod test {
    use super::*;

    #[test]
    fn test_composite_number() {
        composite_number();
    }
}