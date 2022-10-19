use num_bigint::BigInt;

fn self_powers(upper_bound: u32) -> String {
    let mut running_total = BigInt::from(0);
    for i in 1..=upper_bound {
        let power = BigInt::from(i).pow(i);
        running_total += power;
        // println!("{running_total}");
    }
    let running_total_str = running_total.to_string();
    let cut_length = running_total_str.len() - 10;
    let last_10_digits = &running_total_str[cut_length..];
    println!("{last_10_digits}");

    last_10_digits.to_string()

    // let (_, digit_vec) = running_total.to_u32_digits();
    // println!("{digit_vec:?}");
    // let dig_idx = digit_vec.len()-10;
    // println!("{dig_idx}");
    // let last_10 = &digit_vec.as_slice()[dig_idx..];
    // println!("{last_10:?}");
}

mod test {
    use super::*;

    #[test]
    fn test_self_powers_up_to_ten() {
        let last_10 = self_powers(10);
        println!("{last_10}");
        assert_eq!(last_10, "0405071317");
    }

    #[test]
    fn test_self_powers_up_to_ten_thousand() {
        let last_10 = self_powers(1000);
        println!("{last_10}")
    }
}
