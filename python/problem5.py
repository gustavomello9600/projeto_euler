from collections import defaultdict
from functools import reduce
from operator import mul

def break_in_prime_factors(n: int) -> int:
    primes = (2, 3, 5, 7, 11, 13, 17, 19)
    remainder = n
    factors = defaultdict(int)

    for p in primes:
        while remainder % p == 0:
            factors[p] += 1
            remainder //= p
        if p >= n:
            return factors

    return factors


def find_least_number_divisible_by_naturals_up_to_(n):
    factors = defaultdict(int)

    for i in range(2, n + 1):
        factors_of_i = break_in_prime_factors(i)
        for factor in factors_of_i:
            if factors_of_i[factor] > factors[factor]:
                factors[factor] = factors_of_i[factor]

    return reduce(mul, [k**v for k, v in factors.items()])


n = 20

print(f"Least number divisible by all naturals up to {n}: "
      f"{find_least_number_divisible_by_naturals_up_to_(n)}")
    
