function sum_primes_until(n)
    numbers = Array(2:n)
    limit = floor(Int, √n)
    sieve = ones(Bool, n - 1)

    for i in 4:2:n
        sieve[i - 1] = false
    end

    for i in 3:2:limit
        if sieve[i - 1]
            for j in i^2:2*i:n
                sieve[j - 1] = false
            end
        end
    end

    primes = numbers[sieve]
    return sum(primes)
end

println("Sum of primes up to 2,000,000: $(sum_primes_until(2_000_000))")
