use num_bigint::BigInt;
use num_traits::One;
use std::collections::HashMap;

fn add_two_fractions(
    enumerator_a: BigInt,
    denominator_a: BigInt,
    enumerator_b: BigInt,
    denominator_b: BigInt,
) -> (BigInt, BigInt) {
    let new_enumerator_a = enumerator_a.clone() * denominator_b.clone();
    let new_enumerator_b = enumerator_b.clone() * denominator_a.clone();
    let new_denominator = denominator_a.clone() * denominator_b.clone();
    let new_enumerator = new_enumerator_a.clone() + new_enumerator_b.clone();
    (new_enumerator, new_denominator)
}

///Divide an int by a fraction, eg 1 dividid by 5/2 = 2/5
fn divide_by_fraction(n: usize, enumerator: usize, denominator: usize) -> (usize, usize) {
    let new_enuminator = n * denominator;
    (new_enuminator, enumerator)
}

///This, a pure function? NO! It will show you fractions you wouldn't believe
fn expansion_square_root_approximation(
    n: usize,
    cache: &mut HashMap<usize, (BigInt, BigInt)>,
) -> (BigInt, BigInt) {
    let (enumerator_b, denominator_b) = expansion_the_recursive_part(n, cache);

    add_two_fractions(BigInt::one(), BigInt::one(), enumerator_b, denominator_b)
}

///Return a fraction of the nth recursion
/// am I going to cache it? you betcha
fn expansion_the_recursive_part(
    n: usize,
    cache: &mut HashMap<usize, (BigInt, BigInt)>,
) -> (BigInt, BigInt) {
    //base case, just a half
    if n == 1 {
        return (BigInt::one(), BigInt::from(2));
    }
    if let Some(pair) = cache.get(&n) {
        return pair.clone();
    }
    //otherwise  1/(2+1/2)
    // then 1/(2+1/(2+1/2))
    //in general 1(2+SUM(n=2..)(1/2)^(n-1)
    //so this is going above the line eg 1/2 the 1
    let enumerator = 1;
    //start calculating below the line
    //for 2nd expansion we want to get 5/2 2+1/2==(4/2)+(1/2)=5/2
    //for 3rd expansion we want to get 12/5
    //for 4th expansion we want to get 17/12
    let denominator_a: u128 = 2;
    let (enumerator_b, denominator_b) = expansion_the_recursive_part(n - 1, cache);

    //now we add up the 2 with the result on the right
    let below_the_line_enumerator = 2 * denominator_b.clone() + enumerator_b.clone();

    let return_fraction = (denominator_b.clone(), below_the_line_enumerator);
    cache.insert(n, return_fraction.clone());
    return_fraction.clone()
}

///Count expansions where enumerator.#digits > denominator.#digits
fn count_enumerator_victories(upper_bound: usize) -> usize {
    let mut cache = HashMap::<usize, (BigInt, BigInt)>::new();
    let mut count = 0;
    for i in 8..=upper_bound {
        let (enumerator, denominator) = expansion_square_root_approximation(i, &mut cache);
        // println!("{enumerator}/{denominator}");
        let enum_digits = get_digits_bigint(enumerator);
        let denom_digits = get_digits_bigint(denominator);
        if enum_digits.len() > denom_digits.len() {
            count += 1
        }
    }
    count
}

fn get_digits_bigint(number: BigInt) -> Vec<BigInt> {
    let mut big_dig_vec = Vec::new();
    make_big_digit_vec(number, &mut big_dig_vec);
    big_dig_vec
}

fn make_big_digit_vec(n: BigInt, dig_vec: &mut Vec<BigInt>) {
    let ten = BigInt::from(10);
    if n >= ten {
        make_big_digit_vec(&n / &ten, dig_vec);
    }
    let res = n % ten;
    dig_vec.push(res);
}

mod test {
    use super::*;

    use crate::square_root_convergence::add_two_fractions;

    #[test]
    fn test_add_two_fractions() {
        let new_fraction = add_two_fractions(
            BigInt::from(2),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(2),
        );
        let seven = BigInt::from(5);
        let five = BigInt::from(2);
        assert_eq!((seven, five), new_fraction);
    }

    #[test]
    fn test_expansion_the_recursive_part() {
        let mut cache = HashMap::<usize, (BigInt, BigInt)>::new();
        let fract = expansion_the_recursive_part(1, &mut cache);
        assert_eq!((BigInt::one(), BigInt::from(2)), fract);
        let fract = expansion_the_recursive_part(2, &mut cache);
        assert_eq!((BigInt::from(2), BigInt::from(5)), fract);
        let fract = expansion_the_recursive_part(3, &mut cache);
        assert_eq!((BigInt::from(5), BigInt::from(12)), fract);
    }

    #[test]
    fn test_expansion_squareroot_two() {
        let mut cache = HashMap::<usize, (BigInt, BigInt)>::new();
        let fraction = expansion_square_root_approximation(1, &mut cache);
        assert_eq!((BigInt::from(3), BigInt::from(2)), fraction);
        let fraction = expansion_square_root_approximation(2, &mut cache);
        assert_eq!((BigInt::from(7), BigInt::from(5)), fraction);
        let fraction = expansion_square_root_approximation(3, &mut cache);
        assert_eq!((BigInt::from(17), BigInt::from(12)), fraction);
        let fraction = expansion_square_root_approximation(8, &mut cache);
        assert_eq!((BigInt::from(1393), BigInt::from(985)), fraction);
    }

    #[test]
    fn test_count_enumerator_victories() {
        let count = count_enumerator_victories(1000);
        // assert_eq!(1, count);
        println!("Number of expansions where #digits of the enum > #digits of denom: {count}");
    }
}
