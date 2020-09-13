function somar_múltiplos_de_3_e_5_até(n)
    s = 0

    for i in 1:n
        if i % 3 == 0 || i % 5 == 0
            s += i
        end
    end

    return s
end

println(somar_múltiplos_de_3_e_5_até(999))
