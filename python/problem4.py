def find_the_two_greatest_factors_that_multiply_to_a_palindrome():
    diagonal = 0
    palindromes = []

    while True:
        diagonal += 1

        if diagonal % 2 == 1:
            delta = (diagonal + 1)//2
            a_0, b_0 = 1000 - delta, 1000 - delta
        else:
            delta = diagonal//2
            a_0, b_0 = 1000 - delta, 1000 - delta - 1

        for step in range(0, 1000):
            a = a_0 + step
            if a >= 1000:
                break
            b = b_0 - step
            if b < 100:
                break

            product_is_palindrome = check_if_product_is_palindrome(a, b)
            if product_is_palindrome is not None:
                return product_is_palindrome

def check_if_product_is_palindrome(a, b):
    product = a*b
    if product >= 100000:
        digits = 6
    elif product >= 10000:
        digits = 5
    elif product >= 1000:
        digits = 4
    elif product >= 100:
        digits = 3
    else:
        return None

    middle_index = digits//2 if digits != 5 else 3

    for i in range(middle_index):
        if (digit_in_position(i, product)
            != digit_in_position(digits - i - 1, product)):
            return None

    return product, a, b


def digit_in_position(i, number):
    return (number // (10**i)) % 10


print(("{} is the greatest palindrome that is a "
       "multiple of two three-integer numbers, "
       "namely {} and {}").format(
      *find_the_two_greatest_factors_that_multiply_to_a_palindrome()))
