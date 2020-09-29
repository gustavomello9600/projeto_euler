from itertools import cycle

relevant_primes = [3, 7, 11, 13, 17, 19]


def is_prime(n):
    for p in relevant_primes:
        if n % p == 0:
            return False
    return True


p = 21
steps = cycle([2, 4, 2, 2])
soma = sum(relevant_primes) + 7

while p < 2_000_000:
    if is_prime(p):
        relevant_primes.append(p)
        soma += p
    p += next(steps)

print(f"Sum of all primes up to 2,000,000 is: {soma}")
