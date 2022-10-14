fn is_target_triangle(a: u32, p: u32) -> bool {
    (p * (p - 2 * a)) % (2 * (p - a)) == 0
}
/// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.
///{20,48,52}, {24,45,51}, {30,40,50}
/// For which value of p ≤ 1000, is the number of solutions maximised?
fn integer_right_triangle(upper_bound: usize) {
    //we can get the following formula by noting that p must be even
    //b = (p*(p-2a))/(2(p-a))
    let mut max_solutions = 0;
    let mut best_p = 0;
    for p in 10..=upper_bound {
        let mut num_solutions = 0;
        for a in 1..((p / 2) as u32) {
            if is_target_triangle(a, p as u32) {
                num_solutions += 1;
            }
        }
        if num_solutions > max_solutions {
            max_solutions = num_solutions;
            best_p = p;
        }
        num_solutions = 0;
    }
    println!("Perimiter {best_p} has {max_solutions}");
}

mod test {
    use super::*;

    #[test]
    fn test_integer_right_triangle() {
        integer_right_triangle(1000);
    }
}
