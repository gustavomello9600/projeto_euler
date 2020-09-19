use std::collections::HashMap;


const PRIMES: [u8; 8] = [2, 3, 5, 7, 11, 13, 17, 19];


fn main() {
    let n = 20_u8;
    let answer = find_least_number_divisible_by_naturals_up_to_(&n);
    println!("Least number divisible by all naturals up to {} : {}", n, answer);
}

fn find_least_number_divisible_by_naturals_up_to_(n: &u8) -> u64 {
    let mut factors_of_n: HashMap<u8, u8> = HashMap::new();

    for i in 2_u8..(n + 1_u8) {
        let factors_of_i: HashMap<u8, u8> = break_in_prime_factors(i);

        for (factor, factor_multiplicity_in_i) in factors_of_i {
            let factor_multiplicity_in_n: &mut u8 = factors_of_n.entry(factor).or_insert(0_u8);

            if factor_multiplicity_in_i > *factor_multiplicity_in_n {
                *factor_multiplicity_in_n = factor_multiplicity_in_i
            }
        }
    }

    let mut number = 1_u64;
    for (factor, multiplicity) in factors_of_n {
        number *= (factor as u64).pow(multiplicity as u32);
    }

    return number;
}

fn break_in_prime_factors(n: u8) -> HashMap<u8, u8> {
    let mut remainder = n;
    let mut factors = HashMap::new();

    for p in PRIMES.iter() {
        while remainder % p == 0_u8 {
            let multiplicity_of_p = factors.entry(*p).or_insert(0_u8);
            *multiplicity_of_p += 1_u8;
            remainder /= p;
        }

        if p >= &n {
            return factors;
        }
    }

    return factors;
}
