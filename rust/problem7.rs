use std::vec::Vec;

fn main() {
    let mut relevant_primes = vec![3, 7, 11, 13, 17, 19];
    let mut i = 8;

    let mut steps = [2, 2, 4, 2].iter().cycle();
    let mut p = 19;

    while i < 10001 {
        p += steps.next().unwrap();

        if is_prime(p, &relevant_primes) {
            relevant_primes.push(p);
            i += 1;
        }
    }
    
    println!("10001st prime is {}", p);
}

fn is_prime(n: i32, relevant_primes: &Vec<i32>) -> bool {
    for p in relevant_primes.iter() {
        if n % p == 0 {
            return false;
        }
    }
    return true;
}
