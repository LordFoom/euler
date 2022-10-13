#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn is_pentagonal(n:f64) -> bool {
    let test_num:f64 = ((24. * n + 1.).sqrt() + 1.) / 6. ;
    test_num == test_num.floor()
}

fn get_pentagons(number:usize)->Vec<u64>{
    let mut pents = Vec::new();
    for n in 1..number{
        let pn = n*(3*n-1)/2;
       pents.push(pn as u64);
    }
    pents
}
fn pentagon_numbers(){
    let bound = 10000;
    let mut first_num = 0;
    let mut second_num = 0;
    let mut minused = 0;
    let mut plussed = 0;
    let mut distance = u64::MAX;
    //let's generate the first 1000 pentagonal numbers
    let nums = get_pentagons(bound);
    for i in 0..nums.len(){
        let p1 = nums[i];
        for j in i+1..nums.len(){
            let p2 = nums[j];
            let p3 = p2-p1;
            let p4 = p1+p2;
            // println!("Testing {p1} and {p2}");
            if let Ok(min_idx) = nums.binary_search(&p3){
               if let Ok(plus_idx)  =nums.binary_search(&p4){
                   println!("{p2}-{p1}={p3}, the {min_idx} pentagon number");
                   println!("{p1}+{p2}={p4} the {plus_idx} pentagon number");
                   if p4-p3 < distance{
                       first_num = p1;
                       second_num = p2;
                       minused = p3;
                       plussed = p4;
                       distance = p4-p3;
                   }
               }
            }

        }
    }
    println!("{second_num} - {first_num} = {minused}");
    println!("{first_num} + {second_num} = {plussed}");
}

mod test{
    use super::*;

    #[test]
    fn test_is_pentagonal(){
        assert!(is_pentagonal(1.));
        assert!(is_pentagonal(5.));
        assert!(is_pentagonal(12.));
        assert!(is_pentagonal(22.));

        assert!(!is_pentagonal(21.));
        assert!(!is_pentagonal(2.));
    }

    #[test]
    fn test_pentagonal_numbers(){
        pentagon_numbers()
    }

}