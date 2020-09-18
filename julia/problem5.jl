const primes = (2, 3, 5, 7, 11, 13, 17, 19)


function find_least_number_divisible_by_naturals_up_to_(n)
    factors = Dict{Integer, Integer}()

    for i in 2:n
        factors_of_i = break_in_prime_factors(i)
        for factor in keys(factors_of_i)
            if factors_of_i[factor] > get(factors, factor, 0)
                factors[factor] = factors_of_i[factor]
            end
        end
    end

    println(factors)
    return reduce(*, (k^v for (k, v) in factors))
end


function break_in_prime_factors(n)
    remainder = n
    factors = Dict{Integer, Integer}()

    for p in primes
        while remainder % p == 0
            factors[p] = get(factors, p, 0) + 1
            remainder ÷= p
        end

        if p ≥ n
            return factors
        end
    end

    return factors
end


n = 20
answer = find_least_number_divisible_by_naturals_up_to_(n)

println("Least number divisible by all naturals up to $n: $answer")
