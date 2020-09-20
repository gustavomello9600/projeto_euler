from itertools import cycle

relevant_primes = [3, 7, 11, 13, 17, 19]
i = 8


def is_prime(n):
    for p in relevant_primes:
        if n % p == 0:
            return False
    return True


p = 19
steps = cycle([2, 2, 4, 2])

while i < 10001:
    p += next(steps)

    if is_prime(p):
        relevant_primes.append(p)
        i += 1


print(f"10001st prime is {p}")
