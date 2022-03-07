// Find backwards prime numbers from 1 to N. Backward prime numbers are prime numbers whose
// palindrome is also prime eg. 31 <=> 13
use std::collections::HashSet;
fn main() {
    println!("{:?}", backwards_prime(10, 1000000))   
}

fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut backwards_prime_list: Vec<u64> = Vec::new();
    let prime_list = get_prime_list(start, stop);
    
    for prime in prime_list {
        let rev_num = reverse_number(prime);
        if is_prime(rev_num){
            if prime == rev_num {
                continue;
            }
            backwards_prime_list.push(prime);
        }
    }
    
    backwards_prime_list.sort();
    backwards_prime_list
}

fn get_prime_list(start: u64, stop: u64) -> Vec<u64> {
    
    
    if stop <= 2 {
        return Vec::<u64>::new();
    }
    
    let mut is_prime = (2..=stop).collect::<HashSet<u64>>();
    
    // // Remove 0 and 1 if they exist in sequence
    is_prime.remove(&0);
    is_prime.remove(&1);
    

    for i in 2..=(stop as f64).sqrt().floor() as u64 {
        if let Some(_) = is_prime.get(&i) {
            for x in (i*i..=stop).step_by(i as usize) {
                is_prime.remove(&x);
            }
        }
    }
    
    
    let mut prime_list: Vec<u64> = is_prime.into_iter().collect::<Vec<u64>>();   
    prime_list.sort();
    let split_idx = prime_list.iter().position(|&x| x >= start).unwrap() as usize;
    let prime_list = prime_list.split_off(split_idx);
    
    prime_list
    
}

fn is_prime(num: u64) -> bool {
    for i in 2..=(num as f64).sqrt().floor() as u64 {
        if num % i == 0 {
            return false
        }
    }
    true
}

fn reverse_number(num: u64) -> u64 {
    let mut digits = Vec::<u64>::new();
    let mut num_copy = num;
    let mut rev_num: u64= 0;
    
    while num_copy > 0 {
        let digi: u64 = num_copy % 10;
        digits.push(digi);
        num_copy /= 10;
    }
    
    for i in 0..digits.len() {
        rev_num += digits[i] * 10u64.pow((digits.len() -1 - i) as u32);
    }
    rev_num
}
