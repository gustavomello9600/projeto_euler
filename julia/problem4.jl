function find_2_3_digit_factors_of_greatest_palindrome()
    diagonal = 0

    while true
        diagonal += 1

        if diagonal % 2 == 1
            Δ = (diagonal + 1) ÷ 2
            a_0, b_0 = 1000 - Δ, 1000 - Δ
        else
            Δ = diagonal ÷ 2
            a_0, b_0 = 1000 - Δ, 1000 - Δ - 1
        end

        for step in 0:10000
            a = a_0 + step
            if a >= 1000
                break
            end
            b = b_0 - step
            if b < 100
                break
            end
            
            factors_and_palindrome = check_if_product_is_palindrome(a, b)
            if factors_and_palindrome !== nothing
                return factors_and_palindrome
            end
        end
    end
end


function check_if_product_is_palindrome(a, b)
    product = a * b
    if product >= 100000
        digits = 6
    elseif product >= 10000
        digits = 5
    elseif product >= 1000
        digits = 4
    elseif product >= 100
        digits = 3
    else
        return nothing
    end

    middle_index = digits != 5 ? (digits ÷ 2) - 1 : 2

    for i in 0:middle_index
        if (digit_in_position(i, product)
            != digit_in_position(digits - i - 1, product))
            return nothing
        end
    end

    return product, a, b
end


function digit_in_position(i, number)
    return (number ÷ (10^i)) % 10
end

product, a, b = find_2_3_digit_factors_of_greatest_palindrome()
println("> $product is the greatest palindrome that is a multiple of two three-integer numbers, namely $a and $b.")
