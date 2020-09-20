using Base.Iterators: cycle

function find_nth_prime(n)
    relevant_primes = [3, 7, 11, 13, 17, 19]
    i = 8

    steps = cycle((2, 2, 4, 2))
    p = 19
    for step in steps
        p += step

        if is_prime(p, relevant_primes)
            push!(relevant_primes, p)
            i += 1
        end

        if i >= n
            break
        end
    end

    return p
end

function is_prime(n, relevant_primes)
    for p in relevant_primes
        if n % p == 0
            return false
        end
    end
    return true
end

n = 10001
p = find_nth_prime(n)
println("$n prime is $p")
