use std::vec::Vec;

fn main() {
    let mut relevant_primes = vec![3, 7, 11, 13, 17, 19];

    let mut steps = [2, 4, 2, 2].iter().cycle();
    let mut p = 21;
    let mut soma: u128 = 77_u128;

    while p < 2_000_000 {
        if is_prime(p, &relevant_primes) {
            relevant_primes.push(p);
            soma += p as u128;
        }
        p += steps.next().unwrap();
    }

    println!("Sum of all primes up to 2,000,000 is: {}", soma);
}

fn is_prime(n: i32, relevant_primes: &Vec<i32>) -> bool {
    for p in relevant_primes.iter() {
        if n % p == 0 {
            return false;
        }
    }
    return true;
}
