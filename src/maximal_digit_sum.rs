use crate::lychral_numbers::make_digit_vec;
use num_bigint::BigInt;
use num_traits::Zero;

pub fn get_power_digit_sum(a: usize, b: usize) -> BigInt {
    let base_num = BigInt::from(a);
    let pow_num = base_num.pow(b as u32);
    let mut dig_vec = Vec::new();
    make_digit_vec(&pow_num, &mut dig_vec);
    dig_vec.iter().sum()
}

///problem 56, euler
pub fn get_maximal_sum(upper_bound: usize) {
    let mut max_sum = BigInt::zero();
    for a in 1..upper_bound {
        for b in 1..upper_bound {
            let new_sum = get_power_digit_sum(a, b);
            if new_sum > max_sum {
                println!("Found a new maximum for {a}^{b}, which is {new_sum}");
                max_sum = new_sum;
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    pub fn test_get_power_digit_sum() {
        let sum = get_power_digit_sum(10, 100);
        assert_eq!(BigInt::from(1), sum);
    }

    #[test]
    pub fn test_get_maximal_sum() {
        get_maximal_sum(100);
    }
}
