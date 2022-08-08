#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
extern crate core;
use rust_decimal::prelude::*;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use num_bigint::{BigInt, BigUint};
use std::ops::Add;
use num_traits::One;
// use num_traits::Zero;
// pub fn sum_multiples_3_5(){
//     println!("Sum of multiples of 3 and 5 < 1000...");
//     let sum = (1 .. 1000)
//         .filter(|a| a%3==0 || a%5==0)
//         .fold(0, |a, b| a+b);
//     println!("{}", sum);
// }

const ONE: &str = "ONE";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";
const TEN: &str = "ten";
const ELEVEN: &str = "eleven";
const TWELVE: &str = "twelve";
const THIRTEEN: &str = "thirteen";
const FOURTEEN: &str = "fourteen";
const FIFTEEN: &str = "fifteen";
const SIXTEEN: &str = "sixteen";
const SEVENTEEN: &str = "seventeen";
const EIGHTEEN: &str = "eighteen";
const NINETEEN: &str = "nineteen";
const TWENTY: &str = "twenty";
const THIRTY: &str = "thirty";
const FORTY: &str = "forty";
const FIFTY: &str = "fifty";
const SIXTY: &str = "sixty";
const SEVENTY: &str = "seventy";
const EIGHTY: &str = "eighty";
const NINETY: &str = "ninety";
const HUNDRED: &str = "hundred";
const THOUSAND: &str = "thousand";

struct Fibonacci{
    fib_arr: HashMap<usize, BigInt>,
}

impl Fibonacci{
    pub fn new()->Self{
        Self{
            fib_arr:HashMap::with_capacity(100),
        }
    }

    pub fn fibonacci(&mut self, idx:usize)->BigInt{
        println!("Fibonacci number: {}", idx);
        if let Some(x) = self.fib_arr.get(&idx){
            return self.fib_arr[&idx].clone();
        }
        if idx<2 {
            return BigInt::from(1);
        };
        let one_back = self.fibonacci(idx-1);
        let two_back = self.fibonacci(idx-2);
        println!("Number of items in cache: {} ",self.fib_arr.len());
        if !self.fib_arr.contains_key(&idx){
            let fib_num = one_back+two_back;
            println!("Caching {}", fib_num);
            self.fib_arr.insert(idx, fib_num);
        }else{
            println!("{} is in the cache", self.fib_arr[&idx]);
        };
        self.fib_arr[&idx].clone()
    }
}

pub fn is_prime(num_to_check: u64)->bool{
    let factor_found =  !(2..=((num_to_check as f64).sqrt() as u64))
        .any(|i|  num_to_check%i == 0 );
    // println!("Is {} prime? {}", num_to_check, factor_found);
    factor_found
}

// pub fn is_prime_neg_or_pos(num_to_check: u64)->bool{
//     let factor_found =  !(2..=((num_to_check as f64).sqrt() as u64))
//         .any(|i| {
//             let factor = num_to_check%i == 0;
//             // println!("is {} factor for {}", i, num_to_check);
//             factor
//
//         });
//     // println!("Is {} prime? {}", num_to_check, factor_found);
//     factor_found
// }
pub fn print_largest_prime_factors(num_to_check : u64){
    let mut curr_max = 0;
    (2..((num_to_check as f64).sqrt() as u64)).for_each(|i| if num_to_check%i == 0{
        if is_prime(i){
            curr_max = i;
     }
    } );


    println!("{}", curr_max);

}

pub fn is_palindrome(num : u64)->bool{
    let str_num = num.to_string();
    let str_num_reverse = str_num.chars().rev().collect::<String>();
    let is_pal = str_num == str_num_reverse;
    if is_pal {
     println!("Found a palindrome number: {}", str_num)

    }
    // }else{
    //     println!("{} != {}", str_num, str_num_reverse)
    // }
    is_pal
    // println!("Found a palindrome number: "+)
    // for i in 0..str_num.len()/2 {
    //     for j in str_num.len()-1 ..=str_num.len()/2 {
    //         if str_num.chars().nth(i) != str_num.chars().nth(j) {
    //             return false;

    //         }
    //     }
    // }
    // println!("Found a palindrome number: "+)
    // true
}

pub fn largest_palindrome_three_digits(){
    let mut lpn = 0;
    let mut num_1=900;
    let mut num_2=900;

    while num_1 <= 999{
        while num_2 <= 999 {
            let p = num_1 * num_2;
            println!("Product  of {}*{}={}", num_1, num_2, p);
            if is_palindrome(p) && p > lpn {
                lpn = p;
            }
            num_2 += 1;
        }
        num_2=900;
        num_1 += 1;
    }
    println!("largest palindrome prod of 3 digits iiiis: {}", lpn);
}

pub fn smallest_number_all_commonly_divide(up_bound:usize){
    let mut num: u64 = up_bound as u64;
    let mut keep_going = true;
    
    while keep_going 
    {
        let all_divide =  {
            let mut divides = true;
            for i in 2..=up_bound{
                // println!("Testing: {}", i);
                if num % i as u64 != 0{
                    // println!("{} did NOT divide evenly into {}", i, num);
                    divides = false;
                    break;
                }/*else{
                    println!("{} did divide evenly into {}", i, num);
                }*/
            }
            divides
        };
        if !all_divide {
            num +=1
        }else{
            keep_going = false;
        }
    }
    println!("keep_going? {}", keep_going);
    println!("Number with num {}", num);
}

pub fn difference_sumsq_and_sqsum(ub : i64){
    println!("Checking difference for number: {}", ub);
    let sum_of_sq: i64 = (1..=ub)
        .map(|a| a.pow(2))
        .sum();
    let sq_of_sum = (1..=ub).sum::<i64>().pow(2);
    let sum = sum_of_sq - sq_of_sum;
    println!("{}", sum.abs())
}

pub fn nth_prime(num: u32){
    let mut count = 0;
    let mut prime = 2;
    let mut nmb = 1;
    while count < num {
        nmb += 1;
        if is_prime(nmb) {
            count+=1;
            if count == num {
                prime = nmb;
                break;
            }
        } 
    }
    println!("{}", prime);
}

///Sliiiiiiiiding windows
pub fn largest_adjacent_product(num:Vec<u32>, adj_digits: u32){
    let mut product:u64 = 1;
    let mut max_product:u64= 1;
    for j in 0..adj_digits {
        // println!("current c is {}", num[j as usize]);
        product *= num[j as usize] as u64;
        max_product = product;
    } 
    // println!("Product is now {}", product);
    let ub = num.len() as u32 -adj_digits;
        let mut zero_count =0;
    for i in 0..ub{
            // product =1;
        // for j in i..i+adj_digits {
            // println!("current c is {}", num[j as usize]);
            // product *= num[j as usize] as u64;
            // // max_product = product;
        // } 
        // if product > max_product {
            // max_product = product;
        // }
        
        
        let losing = num.get(i as usize).unwrap_or(&1).clone();
        let gaining = num.get((i+adj_digits) as usize).unwrap().clone();

        //slide along 
        if losing > 0 {
            product /= losing as u64;
        }else{ zero_count -= 1; }
        if gaining > 0 {
            product *= gaining as u64;
        }else{ zero_count += 1; }
        if product > max_product && zero_count == 0{
            max_product = product;
            // println!("Setting max to product={}", product);
        }else{
            // println!("Max {} > than product {}", max_product, product);
        }
        
    }
    println!("zerocount: {}", zero_count);
    println!("maxProduct = {} ", max_product);
}

pub fn read_numbers_from_standard_in()->Vec<u32>{
    let mut numbers = String::new();

    std::io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("Could not read from standard in");

    println!("Read in '{}'", numbers);
    numbers.chars()
        .filter(|c| c != &'\n')
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn pythagorean_triplet(){

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    'outer: for i in 1..1000{
        a = i as u64;
        for j in 1..1000{
           b = j as u64;
           let c_sq = a.pow(2) + b.pow(2);

           let c_sqrt = (c_sq as f64).sqrt();
           let c_sqrt_rounded = c_sqrt.round();
           if c_sqrt != c_sqrt_rounded{
               continue;
           }
           c = c_sqrt as u64;
           println!("csq is: {}", c_sq);
           println!("about to check {}+{}+{}", a, b, c);

           if a+b+c==1000
           {
               break 'outer;
           }
           
        }
    }

        println!("Product ({})({})({}) = is: {}", a, b, c, a*b*c);


//     let mut a = 0;
//     let mut b = 0;
//     let mut c = 1_000_000;
//     let mut found = false;
//     for i in 1..1000 {
//         a = (i as u32).pow(2);
//         for j in 2..1000 {
//             b = (j as u32).pow(2);

//             found = a + b == c;
//             if found { break; }
//         }

//         if found { break; }
//     }
//     if found {
//         let sqrt_a = ( a as f64 ).sqrt();
//         let sqrt_b = ( b as f64 ).sqrt();
//         let sqrt_c = ( c as f64 ).sqrt();
//         println!("{} + {} = {}", a, b, c);
//         println!("{}^2 + {}^2 = {}^2", sqrt_a, sqrt_b, sqrt_c);
//         println!("Product ({})({})({}) = is: {}", sqrt_a, sqrt_b, sqrt_c, sqrt_a*sqrt_b*sqrt_c);
//     }else{
//         println!("There was nothing found, meludd");
//     }
}

pub fn sum_of_primes_under(upper_bound:u64){
    
    let mut sum = 2;

    for n in (3..upper_bound).step_by(2){
        if is_prime(n){
            sum +=n;
        }
    }

    println!("Sum of primes under {} is {}", upper_bound, sum)
}

fn largest_product_in_grid(){
    //read in the grid
    // let path = std::path::Path::new("grid.txt");
    let reader = BufReader::new(std::fs::File::open("grid.txt").expect("Cannot open grid file"));
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for (line_no, line) in reader.lines().enumerate(){
        println!("Busy with line:{}", line_no);
        let mut number_line:Vec<i32> = Vec::new();
        line.unwrap().split_whitespace()
            .for_each(|c|{
                number_line.push(c.parse().unwrap());
            });
        matrix.push(number_line);
    }
    //start doing the multiplying
    let n = matrix.len();
    //do the horizontal
    let mut max_prod = 0;
    for row in 0..n{
        for col in 0..n-4{
            //horizontal
            let num1 = matrix[row][col];
            let num2 = matrix[row][col+1];
            let num3 = matrix[row][col+2];
            let num4 = matrix[row][col+3];
            let prod = num1*num2*num3*num4;
            if prod > max_prod{
                max_prod = prod;
            };
        }
    }

        //do the vertical
    for row in 0..n-4{
        for col in 0..n{
            let num1 = matrix[row][col];
            let num2 = matrix[row+1][col];
            let num3 = matrix[row+2][col];
            let num4 = matrix[row+3][col];
            let prod = num1*num2*num3*num4;
            if prod > max_prod{
                max_prod = prod;
            }
        }
    }
    //do the diagonal left to right
    for row in 0..n-4{
        for col in 0..n-4{
            let num1 = matrix[row][col];
            let num2 = matrix[row+1][col+1];
            let num3 = matrix[row+2][col+2];
            let num4 = matrix[row+3][col+3];
            let prod = num1*num2*num3*num4;
            if prod > max_prod{
                max_prod = prod;
            }
        }
    }
    //do the diagonal right to left
    for row in 0..n-4{
        for col in 3..n{
            let num1 = matrix[row][col];
            let num2 = matrix[row+1][col-1];
            let num3 = matrix[row+2][col-2];
            let num4 = matrix[row+3][col-3];
            let prod = num1*num2*num3*num4;
            if prod > max_prod{
                max_prod = prod;
            }
        }

    }
    println!("Max prod: {}", max_prod);
}


fn highly_divisible_triangle_number(){
    //keep going until you find the number
    let mut tn:u64 = 1;
    //let min_divisors:u64 = (1..=500).fold(1, |x,y|x*y);
    // panic!("Just dying to show you min divisors {}", min_divisors);
    // println!("Min divisor number: {}", min_divisors);
    for i in 2.. {
       tn += i;
       if tn%2>0 {//has to be divisible by 500 has to be even
           continue;
       }
       // if tn<min_divisors {
       //     continue;
       // }
       println!("Testing: {}",tn);
       //get divisors
       let mut num_divisors = 0; //1 and itself
       for j in 1..=((tn as f64).sqrt() as u64){
           println!("Possible divisor: {}", j);
           if tn%j == 0{
               num_divisors+=2;
               println!("with {} the num divisors is {}", j, num_divisors);
           };
           if j*j==i {
               num_divisors-=1;
           }
       } 
       // if tn>=76576500{
       //     panic!("We missed it! ");
       // }
           // println!("{}  has {} divisors", tn, num_divisors);
       if num_divisors > 500 {
           println!("{}  has {} divisors, that's over 500", tn, num_divisors);
           break;
       }
    }
}

fn first_digits_of_sum(){
    //read in the  file
    // let reader = BufReader::new(std::fs::File::open("grid.txt").expect("Cannot open grid file"));
    // let mut matrix: Vec<Vec<i32>> = Vec::new();

    let reader = BufReader::new(std::fs::File::open("numbers.txt").expect("Could not open numbers file"));
    // let mut total:BigUint = Zero::zero();
    //for each line, turn it into a number
    // for line in reader.lines(){
    //     let tmp = line.unwrap();
    //     println!("line: {}", tmp);
    //     let curr_num = BigUint::from_str(&tmp).unwrap();
    //     total.add(curr_num);
    // }

    if let Some(bui) = reader.lines().map(|line| BigUint::from_str(&line.unwrap()).unwrap())
        .reduce(|x,y| x.add(y)){
            // println!("{}", bui.to_string()[0..10]);
            let num_str = bui.to_string();
            println!("{}", num_str);
            let mut ten_dig = String::new();
            for (i, c) in num_str.chars().enumerate(){
                if i<10{
                    ten_dig.push(c);
                }else{
                    break;
                } 
            }
            println!("{}", ten_dig);
        }else{
            println!("We got nothing");
        };
    //get the first 10 digits by...?shifting?
}

fn collatz(n : i64)->i64{
        let new_n = if n%2 == 0{
            n/2            
        }else{
            3*n+1 
        };
        // println!("->{}", new_n);
        new_n

}
fn collatz_loop(){

    let mut collatz_cache:HashMap<i64,i64> = std::collections::HashMap::new();
    let mut max_chain_len = 0;
    let mut curr_chain_len = 1;
    let mut max_chain_num = 0;
    for n in 13..1_000_000{
    // for n in 13..14 {
        let mut curr_num = n;
        while curr_num != 1{
            let old_num = curr_num.clone();
            if collatz_cache.contains_key(&curr_num) {
                println!("Curr num {} IS in the cache, with value{}!", curr_num, collatz_cache[&curr_num]);
                curr_chain_len += collatz_cache[&curr_num];
                if curr_chain_len > max_chain_len {
                    max_chain_len = curr_chain_len;
                    max_chain_num = n;
                }else{
                    curr_chain_len = 0;
                }
                curr_num = 1;
                continue;
            }else{
                println!("We are doing it fresh, curr_num starts as {}, storing it", curr_num);
                 collatz_cache.entry(curr_num).or_insert(0);
                curr_num = collatz(curr_num);
                curr_chain_len += 1;
                 if let Some(num) = collatz_cache.get_mut(&curr_num){
                    *num = curr_chain_len;
                 }
                println!("curr_num ={}, curr_chain_len = {}", curr_num, curr_chain_len);
            }
            println!("{} old_num => {} new_num", old_num, curr_num);
            if curr_chain_len > max_chain_len {
                max_chain_len = curr_chain_len;
                max_chain_num = n;
             }//else{

            //     curr_chain_len = 0;
            // }
        }
        curr_chain_len = 0;
    }
    println!("{} has longest chain = {} ", max_chain_num, max_chain_len);
}

fn collatz_try_again(){
    let mut seq_len: u64 = 0;
    let mut start_num: u64 = 0;
    let mut seq: u64 = 0;
    let mut cache: [u64;1_000_001] = [0;1_000_001];
    cache[1]=1;
    for i in 2..1_000_000{
        let mut k = 0;
        // seq = i;
        let mut seq: u64 = 0;
        while seq != 1 && seq >=i {
            k+=1;
            if seq %2 == 0{
                seq = seq/2;
            }else{
                seq = (3*seq)+1;
            }
        }
       cache[i as usize] = k+cache[seq as usize];
       if cache[i as usize] > seq_len {
           seq_len = cache[i as usize];
           start_num = i;
       }
   }

   println!("{} has a chain of {}", start_num,  seq_len);
}

fn count_grid_routes(size:usize){
    // let wid;
    // let height = 20;
    // let mut grid = vec![vec![0; width][height]];
    // let mut grid = vec![vec![0;size][size]];
    // let height = size as u32;
    // let width = size as u32;
    // grid[0][0] = 1;
    // for i in 0..=width{
    //     for j in 0..=size{
    //         if i>0{
    //             grid[i][j] += grid[i-1][j];
    //         }
    //         if j>0 {
    //             grid[i][j] += grid[i][j-1];
    //         }
    //     }
    // }
    let mut grid: [[u64;21];21] = [[0;21];21 ];
    grid[0][0]=1;
    for i in 0..=size{
        for j in 0..=size{
            if i>0{
                grid[i][j] += grid[i-1][j];
                println!("Got grid[{}][{}]={}", i, j, grid[i][j]);
            }
            if j>0 {
                grid[i][j] += grid[i][j-1];
                println!("Got grid[{}][{}]={}", i, j, grid[i][j]);
                }
            }
    }
    println!("Here comes the answer! {}", grid[20][20]);
    //if less that width, increase width by ONE - recursing?
    //if less than height, increase height by ONE - recursing?
    //when done add everything together
}

fn sum_digites(n:u32){

    //we take 2 to the power of the digit
    let base = BigUint::from(2 as u64);
    let big_num = base.pow(n);
    let big_num_str = big_num.to_string();
    let sum:u64 = big_num_str.chars()
        .into_iter()
        .fold(0, |a,b| {
            if let Some(d) = b.to_digit(10) {
                println!("adding {} to {}",d, a);
                a + d as u64
            } else {
                panic!("Should have worked!");
            }
        });

    println!("");
    println!("Herewith the sum: {}", sum)
}


fn number_letter_count(){

    let mut count = 0;
    for i in 1..=1000 {
    // for i in 1..=5 {
        let number_word = make_word(i);
        println!("{}. {}", i, number_word);
        count +=  number_word.len() as i32;
    }

    println!("COUUUUNT IT: {}", count);

}


fn make_word(n: i32)->String{
    // println!("n is {}", n);
    if n<10 {
        match n{
            1 => return ONE.to_string(),
            2 => return TWO.to_string(),
            3 => return THREE.to_string(),
            4 => return FOUR.to_string(),
            5 => return FIVE.to_string(),
            6 => return SIX.to_string(),
            7 => return SEVEN.to_string(),
            8 => return EIGHT.to_string(),
            9 => return NINE.to_string(),
            _ => panic!("Should not have gotten here"),
        }
    }

    if n < 100 {
        if n < 20{
            match n {
                10 => return TEN.to_string(),
                11 => return ELEVEN.to_string(),
                12 => return TWELVE.to_string(),
                13 => return THIRTEEN.to_string(),
                14 => return FOURTEEN.to_string(),
                15 => return FIFTEEN.to_string(),
                16 => return SIXTEEN.to_string(),
                17 => return SEVENTEEN.to_string(),
                18 => return EIGHTEEN.to_string(),
                19 => return NINETEEN.to_string(),
                _ =>  panic!("Should not have got here"),
}        };

       // get first digit, get ty word for it
        let x = n.to_string().chars().nth(0).unwrap();
        let first_digit = x as i32 - 48;
        let first_word = match first_digit{
            2 => TWENTY,
            3 => THIRTY,
            4 => FORTY,
            5 => FIFTY,
            6 => SIXTY,
            7 => SEVENTY,
            8 => EIGHTY,
            9 => NINETY,
            _ => panic!("Should not have gotten here 2"),
        };
        //get second digit and add it
        let second_digit = n.to_string().chars().nth(1).unwrap() as i32 - 48;
        let second_word = if second_digit > 0 {
            make_word(second_digit)
        }else{
            String::new()
        };
        return format!("{}{}", first_word, second_word);
    }


    if n < 1000 {
        let first_digit = n.to_string().chars().nth(0).unwrap() as i32 - 48;
        let prefix = make_word(first_digit);
        let first_word = format!("{}hundred", prefix);
        let rest_of_number = str::parse(&n.to_string()[1..]).unwrap();
        let rest_of_number_word = if rest_of_number>0 {
         format!("and{}", make_word(rest_of_number))
        }else{
            String::new()
        };
        return format!("{}{}", first_word, rest_of_number_word);
    }
    String::from("onethousand")
}

fn max_path_sum(){

    let  triangle = include_str!("triangle1.txt");
    let mut graph: Vec<Vec<u64>> = triangle
        .lines()
        .map(|line| line.split(' ')
            .map(|numb| numb.parse::<u64>().unwrap())
            .collect())
        .collect();

    for row in (0 .. graph.len()-1).rev() {
        for column in 0 .. graph[row].len() {
            graph[row][column] += std::cmp::max(graph[row+1][column], graph[row+1][column+1]);
        }
    }
    println!("max path: {}", graph[0][0]);
}

///How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
fn count_sundays(){
    //    1 Jan 1900 was a Monday.
    //     Thirty days has September,
    //     April, June and November.
    //     All the rest have thirty-one,
    //     Saving February alone,
    //     Which has twenty-eight, rain or shine.
    //     And on leap years, twenty-nine.
    //     A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

    //if 1 jan 1900 is a monday, 1 jan 1901 is a tuesday
    let mut curr_year = 1901;
    let mut curr_day_of_week =2;
    let mut curr_day_of_month = 1;
    let mut curr_month_of_year = 1;
    let mut sunday_count = 0;
    let mut curr_last_day_of_month = 31;
    while curr_year < 2000{
        let mut curr_day_of_year = 1;
        let  leap_year =
            if curr_year % 4  == 0 {
                if curr_year % 100 == 0{
                    if curr_year % 400 == 0 {
                        true
                    }else{
                        false
                    }
                }else{
                    true
                }
            }else {
                false
            };
        let days_in_this_year = if leap_year {
            366
        }else{
            365
        };

        while curr_day_of_year <= days_in_this_year{
            //this is on the day
           if curr_day_of_month == 1 && curr_day_of_week == 1 {
               sunday_count+=1;
           }
            //now we prepare for the next day
            curr_day_of_week += 1;
            if curr_day_of_week > 7 {
                curr_day_of_week = 1;
            }
            curr_day_of_month += 1;
            if curr_day_of_month > curr_last_day_of_month {
                curr_day_of_month = 1;
                curr_month_of_year += 1;
                if curr_month_of_year > 12 {
                    curr_month_of_year = 1;
                }
                curr_last_day_of_month = get_days_in_month(curr_month_of_year, leap_year);
            }

            curr_day_of_year += 1;

        }
        curr_year += 1;
    }
    println!("So many sundays! {} to be precise...", sunday_count);
}

fn factorial_digit_sum(n: u32){
    let mut curr_dig = n;
    let base = BigUint::from(n);
    let mut count = BigUint::one();
    while curr_dig>0{
       count*=curr_dig;
       curr_dig-=1;
    }
    println!("100! = {}", count);

    let mut dig_sum = 0;
    count.to_string().chars()
        .for_each(|c| {
            let num = c.to_digit(10).unwrap();
            dig_sum+=num;
        });
    println!("Dig sum = {}", dig_sum);
}

///
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
/// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.
fn amicable_numbers(){
    let mut divisor_sum_cache=HashMap::<i32, i32>::new();

    //this is a cache of number to divisors, eg 10 => 1, 2, 5
    let mut divisor_cache = BTreeMap::<i32, Vec<i32>>::new();

    let mut checked_nums = BTreeSet::new();
    let mut amicable_nums = Vec::new();
    for i in 2..10000{
        // println!("Checking {}", i);
        if checked_nums.contains(&i){
            // println!("{} already been checked", i);
            continue;
        }
        //get this number divisors
        let first_num_div = get_divisors(i, &mut divisor_cache);
        //sum them
        let first_sum = first_num_div.into_iter().sum();
        // println!("First sum is: {}", first_sum);
        if i == first_sum{
            // amicable_nums.push(i);
            continue;
        }
        //get summed number
        //get its divisors
        let second_sum_div = get_divisors(first_sum, &mut divisor_cache);
        //sum
        let second_sum = second_sum_div.into_iter().sum();
        // println!("Second sum is: {}", second_sum);
        //do they match each other?
        if i == second_sum{
            println!("Found amicable numbers: {} and {}", i, first_sum);
            //if so, we found amicable numbers!
            amicable_nums.push(i);
            amicable_nums.push(first_sum);
            checked_nums.insert(first_sum);
        }
    }

    let amicable_num_sum:i32 = amicable_nums.into_iter().sum();
    println!("The sum of allll those amicables: {}", amicable_num_sum);

}
// use num_integer::roots::Roots;
fn get_divisors(num:i32, divisor_cache:&mut BTreeMap<i32, Vec<i32>>)->Vec<i32>{

    let mut divisors_to_sum = Vec::new();
    let bound = (num as f64).sqrt() as i32;

    //do we make a collection of values to check?
    //let's initialize, we build all the rvariables from 1 to n^1/2
    // for i in 1..=num.sqrt() {
    //    divisors_to_sum.push(i);
    // }
    //then we go from the largest N that is <= square root and if it is a factor, then add it and all its factors to divisors, remove from list to check
    // divisors_to_sum.last()

    //check all other values that are <= square root that haven't been removed
    // for key in divisors.keys
    divisors_to_sum.push(1);// one goes into everything, baby
    for i in 2..bound {
        //if
        if num%i == 0{
            divisors_to_sum.push(i);
            divisors_to_sum.push(num/i);
        }
    }
    divisor_cache.insert(num, divisors_to_sum.clone());
    divisors_to_sum
}


//     Thirty days has September,
//     April, June and November.
//     All the rest have thirty-one,
//     Saving February alone,
//     Which has twenty-eight, rain or shine.
//     And on leap years, twenty-nine.
fn get_days_in_month(month: i32, leap_year: bool) -> i32{
    match month{
        2 => if leap_year{
            29
        }else{
            28
        },
        9|4|6|11 => 30,
        _ => 31,
    }
}

fn calc_name_score(line_no:usize, name:&str)->i32{
    let summed:i32 = name.chars()
                     .map(|c| c as i32 - 64)
                     .sum();
    println!("{} is the sum, {} is the line_no", summed, line_no);
    return summed*line_no as i32;
}

fn name_scores(){
    // let name_score:i32 = include_str!("p022_names.txt")
    //     .split(",")
    //     .map(|name| name.replace("\"", ""))
    //     .enumerate()
    //     .map(|(line_no, name)| calc_name_score(line_no+1, &name))
    //     .sum();

    let names = include_str!("p022_names.txt");
    let mut name_sorted = names
        .split(",")
        .map(|name| name.replace("\"", ""))
        .collect::<Vec<String>>();
    name_sorted.sort();
    let mut line_no = 0;
    let name_score:i32 = name_sorted.iter()
                                .map(| name| {
                                    line_no += 1;
                                    calc_name_score(line_no, &name) })
                                .sum();
    println!("Is this the score you wanted???? {} ", name_score);
}


fn abundant_sums(){

   //As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16,
    // the smallest number that can be written as the sum of two abundant numbers is 24.
    // By mathematical analysis, it can be shown that all integers greater than 28123
    // can be written as the sum of two abundant numbers. However, this upper limit cannot be
    // reduced any further by analysis even though it is known that the greatest number that
    // cannot be expressed as the sum of two abundant numbers is less than this limit.
    //
    // Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

    //so everthing smaller than 23 cannot be written as the sum of two abundant numbers
    //lets get all the abundant numbers < 28123, and then we test every number to see if that number minus abundant number is also abundant number
    let abundant = calc_abundant_numbers(28123);
    let mut nas:BTreeSet<i32> = (1..=23 as i32).collect();
    for i in 24..=20161{
        if !is_sum_of_two_abundant_numbers(i, &abundant){
            println!("NOT the sum of two abundant numbers = {}", i);
            nas.insert(i);
        }
    }

    println!("Number of all positivive integers which cannot be written as\
     the sum of two abundant number is = {}, sum  = {}", nas.len(), (nas.iter().sum::<i32>() ));

}

fn is_sum_of_two_abundant_numbers(num:i32, abundant: &BTreeSet<i32>)->bool{
    // println!("Checking to see if {} is the sum of two abundant numbers....", num);
    for i in abundant{
        if i > &num{
            break;
        }
        let rem = num - i;
        if &rem == i || abundant.contains(&rem){
            // println!("{} is the sum of two abundant numbers: {}+{} ", num, rem, i);
            return true;
        }
    }
    return false;
}

fn calc_abundant_numbers(upper_bound:i32) -> BTreeSet<i32>{
    if upper_bound<24{
        return BTreeSet::new();
    }

    let mut abundant_numbers = BTreeSet::new();
    for i in 12..=upper_bound {
        let divisors = get_divs(i);
        let factor_sum:i32 = divisors.iter().sum();
        // divisors.iter().for_each(|d| println!("{}",d));
        // println!("This is the sum: {}", factor_sum);
        // panic!("Force stop on: {}", i);
        if factor_sum > i {
            abundant_numbers.insert(i);
        }
    }
    println!("Found {} abundant numbers ", abundant_numbers.len());
    for i in &abundant_numbers{
        println!("{}", i);
    }
    abundant_numbers
}

fn get_divs(num:i32)->Vec<i32>{

    let mut divisors_to_sum = Vec::new();
    let bound = (num as f64).sqrt() as i32;
    // println!("Bound is {}", bound);
    //then we go from the largest N that is <= square root and if it is a factor,
    // then add it and all its factors to divisors, remove from list to check
    // divisors_to_sum.last()

    //check all other values that are <= square root that haven't been removed
    // for key in divisors.keys
    divisors_to_sum.push(1);// one goes into everything, baby
    for i in 2..=bound {
        //if
        if num%i == 0{
            divisors_to_sum.push(i);
            let counter_factor = num / i;
            if i != counter_factor{//don't put eg 4 in twice when factoring 16
                divisors_to_sum.push(counter_factor);
            }
        }
    }
    // divisor_cache.insert(num, divisors_to_sum.clone());
    divisors_to_sum
}

fn construct_lexigraphical_set(digits: &mut Vec<char>) {
     //loop through the digits - we need to go from smallest possible value, to biggest value,
    //monotonically
    let num_dig = digits.len();
    let mut count = 1;
    let n = digits.len();
    while count<1_000_000 {
        println!("Count is {}", count);
        println!("Digits are {} ", digits.iter().collect::<String>());

        if n<=1{
            break;
        }
        let mut end_of_sequence = n - 1;//last element
        //find the element where the last element is the biggest
        while end_of_sequence > 1 && digits[end_of_sequence] < digits[end_of_sequence-1]{
            end_of_sequence -= 1;
        }
        if end_of_sequence == 1{
            break;
        }

        //now we have a big one on right, and small one on left, we want on the one on the left, we're currently on the right

        let pivot = end_of_sequence-1;
        // end_of_sequence -= 1;

        //we find the longest string from 1,2,...,N
        let mut right_most_successor= end_of_sequence;
        while right_most_successor < n-1 && digits[right_most_successor] > digits[pivot] {
           right_most_successor +=1 ;
        }
        //now we swap
        println!("About to swap: {} with {}", digits[pivot], digits[right_most_successor]);
        digits.swap(pivot, right_most_successor);
        //now we reverse the subsequence from just after the pivot to the end
        let mut temp_vec = digits.drain(0..=pivot).collect::<Vec<char>>();
        digits.reverse();
        temp_vec.append(digits);
        *digits = temp_vec;
        count += 1;
    }

    println!("{}th lexicographic permutation is {}", count, digits.iter().collect::<String>());
    //only down to 1 because we are going to subtract 1 in there
    // for digit in (1..num_dig).rev() {
    //
    //    //swap the last digit with next-to-last digit
    //     let prev = digit-1;
    //     // let mut new_vec = digits.clone();
    //     new_vec.swap(digit, prev);
    //     let tmp:String = new_vec.clone().into_iter().collect();
    //     println!("We have swapped: {}", tmp);
    //     //add it to the lex strings
    //     let swap_str = new_vec.clone().into_iter().collect();
    //     // lex_str.push(swap_str);
    //     //if num digits to right > 2, then recurse on this method,
    //     if num_dig - prev > 2{
    //         //we want to get the digits remaining on the right, in order
    //         // let sub_vec = new_vec[digit..].iter().collect();
    //         let sub_vec = new_vec[digit..].to_vec();
    //        let mut sub_set = construct_lexigraphical_set(&sub_vec);
    //         //get the substring from beginning to end
    //         let prefix: String = new_vec[0..digit].into_iter().collect();
    //         for sub_fix in sub_set {
    //             let new_lex_str = format!("{}{}", prefix, sub_fix);
    //             println!("Pushing in the string {}", new_lex_str);
    //             //pushing in the new string
    //             lex_str.push(new_lex_str);
    //             // num_counter += 1;
    //         }
    //     }else{
    //         println!("Pushing in the end res string {}", swap_str);
    //         lex_str.push(swap_str);
    //         // num_counter += 1;
    //     }
    //
    //     if lex_str.len() >= 1_000_000{
    //         println!("{}th is: {}", lex_str.len(), lex_str.last().unwrap());
    //         return lex_str;
    //
    //     }
    // }
}

fn construct_lexicagraphical_permutation(digits: &mut Vec<char>) -> bool{

    //we find the longest sequence in order - since it started sorted, we can check from the right
    let n = digits.len();
    if n<=1 {
        false;
    }
    let mut start_marker = n-1;
    while start_marker > 0 && digits[start_marker] < digits[start_marker-1]{
        start_marker -= 1;
    }
    // println!("The start marker is: {}", start_marker);
    //we now get the pivot, just to the left of the start marker, and find the rightmost successor
    if start_marker <= 0{
        return false;//run out of permutations
    }
    let pivot = start_marker -1;
    let mut end_marker = n -1;
    while digits[end_marker] < digits[pivot] {
        end_marker -= 1;
    }

    //now we swap
    digits.swap(pivot, end_marker);
    //now we reverse the right hand section
    digits[start_marker..].reverse();
    // println!("This is the  permutation: {}", digits.iter().collect::<String>());
    return true;
}

// fn longest_recurring_cycle(upper_bound:usize){
//
//     let is_power_of_two = |x: usize| -> bool { x!=0 && ((x & (x-1)) == 0) };
//     let mut longest_string = String::new();
//     for d in 2..upper_bound{
//         //skip power of 2
//         // if is_power_of_two(d){
//         //     continue;
//         // }
//         //skip multiple of 2 or 5
//         if d%2 == 0 || d%5 == 0{
//             continue;
//         }
//         let decimal:f64 = 1./(d as f64);
//
//         let mut first_test_char = ' ';
//
//         let mut first_str = String::new();
//         let mut second_str = String::new();
//         let mut current_index = 2;//this is the index that we use to
//         let mut possible_start_found = false;
//         for (i,c) in decimal.to_string().chars().enumerate() {
//             //skip the first two characters, which will be 0 and .
//             if i == 0 || i == 1{
//                 continue;
//             }
//             if i == 2{
//                 first_str = String::from(c);
//                 first_test_char = c;
//                 continue;
//             }
//             if current_index >= first_str.len(){
//                 break;
//             }
//             //building up the cycle
//             let test_char = first_str.chars().nth(current_index).unwrap();
//             println!("Current index is {}", current_index);
//             println!("Test char is: {}", test_char);
//             println!("First test char is: {}", first_test_char);
//             if test_char == c && !possible_start_found {
//                 if current_index > 2 && c == first_test_char{
//                     println!("Found possible start, second string is {}", second_str);
//                    possible_start_found = true;
//                 }else{
//                     println!("Building second string...");
//                     second_str.push(c);
//                     current_index += 1;
//                 }
//             }
//             if possible_start_found {
//                 if first_str == second_str{
//                     println!("Found cycle!");
//                     if first_str.len() > longest_string.len(){
//                         longest_string = first_str.clone();
//                     }
//                 }
//                 break;
//             }else {
//                 // println!("Building first string...");
//                first_str.push(c);
//                current_index+=1;
//             }
//             // print!("{}", c);
//        }
//         println!();
//
//    }
//     println!("Longest cycle is {} with length {}", longest_string, longest_string.len());
// }

fn longest_recurring_cycle(upper_bound:usize) -> (usize, usize) {

    let mut seq_len = 0;
    let mut seq_num = 1;
    for i in 3..upper_bound{
        if i%2 == 0 || i%5 ==0{
            continue;
        }

        let mut remainder_count= 0;
        let mut remainder = 1;
        while remainder != 0 && remainder_count<upper_bound{
            // println!("Remainder: {}", remainder);
            remainder = remainder*10 % i;
            remainder_count += 1;
            if remainder == 1{
                // println!("Found cycle with length: {}")
                if remainder_count > seq_len{
                    seq_len = remainder_count;
                    seq_num = i;
                }
                break;
            }
        }
    }
    (seq_num, seq_len)
}

fn quadratic_expression(){
    //Considering quadratics of the form:
    // n2+an+b, where abs(a)<1000 and abs(b)<= 1000
    // , where and
    //
    //
    // Find the product of the coefficients,
    // a and b , for the quadratic expression that produces the maximum number of
    // primes for consecutive values of , starting with n = 0

    let mut longest_seq_count = 0;
    let mut longest_seq_a = 0;
    let mut longest_seq_b = 0;
    let mut is_continuous_prime = true;
    let mut n:i64 = 0;
    let mut curr_seq_count = 0;
        for a in -999..1000{
            for b in -1000..=1000{
                //from n = 0 we know that b must be a prime
                if !is_prime(b.abs() as u64){
                   continue;
                }
                //if b is 2, then a  must be even to make an odd number ( all prime but 2 is odd)
                if b.abs() == 2 && a%2 != 0{
                    continue;
                }else if b.abs()!=2 && a%2 == 0{//if b is any other prime, we need another odd, so that a+b+1 is odd and a possible prime
                    continue;
                }

                //generate the primes
                while is_continuous_prime{
                    println!("n is {}", n);
                    println!("Testing {0}^2 +({1}){0} + ({2})  ", n, a, b);
                    let num:i64 = n.pow(2) + a*n +b;
                    if is_prime(num.abs() as u64){
                        curr_seq_count += 1;
                        if curr_seq_count > longest_seq_count{
                            longest_seq_count = curr_seq_count;
                            longest_seq_a = a;
                            longest_seq_b = b;
                        }
                        n +=1;
                    }else{
                        curr_seq_count = 0;
                        n = 0;
                        is_continuous_prime = false;
                    }
                }
                println!("Looping out");
                //reset so we can trigger the loop again
                is_continuous_prime = true;
            }
    }
    println!("For a = {} and b = {} we get a sequence of primes that is {} long, the product of coefficents is {}",
             longest_seq_a, longest_seq_b, longest_seq_count, longest_seq_a*longest_seq_b);
}

///Count diagonals of an n*n spiral square
fn spiral_sum(n:usize){
    println!("Spiral sum!");
    let mut curr_ring = 0;
    let mut curr_len = 0;
    let mut curr_gap = 0;
    let mut diag_sum = 0;
    let mut corner_val = 1;
    while curr_len < n {

        println!("curr_ring = {}", curr_ring);
        if curr_ring == 0{
            println!("adding 1");
            curr_ring += 1;
            curr_len +=1;
            diag_sum += corner_val;
            continue;
        }
        curr_len = curr_ring * 2 + 1;
        curr_gap = curr_ring * 2 ;
        for i in 1..= 4{
            println!("current corner val: {}", corner_val);
            corner_val += curr_gap;
            println!("new corner val: {}", corner_val);
            println!("Diagonal sum: {}", diag_sum);
            diag_sum += corner_val;
            println!("New diagonal sum: {}", diag_sum);
        }

        curr_ring +=1;
    }
    println!("Diagonal sum is {}", diag_sum);
}

///How many distinct terms are in the sequence generated by ab for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
pub fn distinct_powers(bound: usize) -> usize{
    let mut ordered_set = BTreeSet::new();
    for a in 2 ..= bound{
       for b in 2..= bound{
           let big_a = BigUint::from(a);
           // let big_b = BigUint::from(b);
           let curr_num: BigUint = big_a.pow(b as u32);
          ordered_set.insert(curr_num);
       }
    }
    ordered_set.len()
}

fn main() {


}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    pub fn test_calc_name_score(){
       let score = calc_name_score(938, "COLIN");
        assert_eq!(49714, score);
    }

    #[test]
    pub fn test_construct_lex_numbers(){
        let mut digits = vec!['0', '1','2','3','4','5','6','7','8','9',];
        // let mut digits = vec!['0', '1','2','3',];
        let mut count = 1;
        while count<1_000_000 && construct_lexicagraphical_permutation(&mut digits){
            count +=1;
        }
        println!("This is what we end up with at permutation {}: {} ", count, digits.iter().collect::<String>());

    }

    #[test]
    pub fn digit_fibonacci_with_1000() {
        let mut digit_count = 1;
        let mut curr_fibo_idx = 0;
        let mut curr_fibo = BigInt::from(0);
        let mut fib = Fibonacci::new();

        while digit_count < 1000 {
            println!("Getting {} fibonacci number, ", curr_fibo_idx);
            let new_fib = fib.fibonacci(curr_fibo_idx);
            println!("Fib number is: {} ", new_fib);
            digit_count = new_fib.to_string().chars().count();
            curr_fibo_idx +=1;
            curr_fibo = new_fib;

        }
        println!("First dibonacci with 1000 digits: {}, which is the {}th number", curr_fibo, curr_fibo_idx);

    }

    #[test]
    pub fn fractions(){
        let (cycle_num, cycle_len) = longest_recurring_cycle(1000);
        println!("Length of longest decimal: {}, which is for number: {}", cycle_len, cycle_num);
        // println!("{}", 1./983. );
    }

    #[test]
    pub fn quadratic_formula_detection(){
        quadratic_expression();
    }

    #[test]
    pub fn calc_spiral_sum(){
        spiral_sum(1001)
    }

    #[test]
    pub fn distinct_powers_yeah(){
        println!("{}", distinct_powers(100));
    }
}
