function somar_ímpares_de_fibonacci_até(n)
    n_current = 1
    n_before = 0

    s = 0

    while n_current < n
        if n_current % 2 == 1
            s += n_current
        end

        n_current, n_before = n_current + n_before, n_current
    end
    return s
end

println("Soma: $(somar_ímpares_de_fibonacci_até(4000000))")
