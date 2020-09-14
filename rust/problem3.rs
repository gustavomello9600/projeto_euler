fn main() {
    let n: i64 = 600851475143;
    
    let mut found_primes: Vec<i64>  = vec![2];
    let mut current_prime: i64 = 2;

    let mut remaining_factors = n;
    let mut greatest_prime_factor_found = 1;

    while remaining_factors != 1 {
        let mod_over_current_prime = remaining_factors % current_prime;
        if mod_over_current_prime == 0 {
            greatest_prime_factor_found = current_prime;
            while remaining_factors % current_prime == 0 {
                remaining_factors /= current_prime;
            }
        }

        for i in (current_prime + 1)..(n + 1) {
            let mut i_is_prime = true;
            for p in &found_primes {
                if i % p == 0 {
                    i_is_prime = false;
                    break
                }    
            }

            if i_is_prime {
                found_primes.push(i);
                current_prime = i;
                break
            }
        }
    }

    println!("Maior fator primo de {}: {}", n, greatest_prime_factor_found)
}
