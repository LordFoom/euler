use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

///Calculate number of calculations
/// Where set_to_take is shorthand for set [1..=set_to_take_from]
/// and num_to_take is size of combination to take
/// n is set to take from
/// r is how many to take
fn calc_combinations(n: usize, r: usize) -> BigUint {
    factorial(n) / (factorial(r) * (factorial(n - r)))
}

fn factorial(n: usize) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}

fn count_combinations() {
    let mut combination_count = 0;
    let one_million = 1_000_000.to_biguint().unwrap();
    for i in 23..=100 {
        for n in 3..i {
            let num_combinations = calc_combinations(i, n);
            if num_combinations > one_million {
                println!("({i},\n {n}) has {num_combinations} combinations, which is > 1,000,000");
                combination_count += 1;
            }
        }
    }

    println!(
        "Number of combinations greater than OOOOONE MIIILLLION DOLLLAAARS: {combination_count}"
    );
}
mod test {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_factorial() {
        let fact = factorial(10);
        assert_eq!(3628800.to_biguint().unwrap(), fact);
    }

    #[test]
    fn test_combinations() {
        let c1 = calc_combinations(5, 3);
        assert_eq!(10.to_biguint().unwrap(), c1);
        let c2 = calc_combinations(23, 10);
        assert_eq!(1144066.to_biguint().unwrap(), c2);
        let c2 = calc_combinations(100, 10);
        let c2 = calc_combinations(100, 99);

        let new_c = calc_combinations(99, 93);
        println!("NEW C: {new_c}");
    }

    #[test]
    fn test_count_combinations() {
        count_combinations()
    }
}
