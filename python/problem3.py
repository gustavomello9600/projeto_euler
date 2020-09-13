from typing import List, Tuple


def fatores_primos_de_(n: int) -> int:
    fatores: List[Tuple[int, int]] = []
    fatores_remanescentes = n

    primos_encontrados = [2]
    primo = 2

    while fatores_remanescentes != 1:
        primo_é_fator_de_n = fatores_remanescentes % primo == 0
        if primo_é_fator_de_n:
            multiplicidade = 0
            while fatores_remanescentes % primo == 0:
                multiplicidade += 1
                fatores_remanescentes = fatores_remanescentes // primo
            fatores.append((primo, multiplicidade))

        for i in range(primo + 1, n + 1):
            i_é_primo = True
            for p in primos_encontrados:
                i_possui_fator_primo = i % p == 0
                if i_possui_fator_primo:
                    i_é_primo = False
                    break
            if i_é_primo:
                primo = i
                primos_encontrados.append(primo)
                break

    return fatores

print(fatores_primos_de_(600851475143))
