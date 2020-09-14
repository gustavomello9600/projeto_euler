function greatest_prime_factor_of_(n::Int64)
    found_primes = Array{Int64}([2])
    current_prime::Int64 = 2

    remanescent_factors::Int64 = n
    greatest_prime_factor_found::Int64 = 1

    while remanescent_factors ≠ 1
        current_prime_is_a_factor_of_n = remanescent_factors % current_prime == 0
        if current_prime_is_a_factor_of_n
            greatest_prime_factor_found = current_prime
            while remanescent_factors % current_prime == 0
                remanescent_factors ÷= current_prime
            end
        end

        for i in (current_prime + 1):(n + 1)
            i_is_prime = true
            for prime in found_primes
                if i % prime == 0
                    i_is_prime = false
                    break
                end
            end
       
            if i_is_prime
                push!(found_primes, i)
                current_prime = i
                break
            end
        end
    end

    greatest_prime_factor_found
end

println(greatest_prime_factor_of_(600851475143))
