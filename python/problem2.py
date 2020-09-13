n_atual = 1
n_anterior = 0

soma = 0
while n_atual < 4000000:
    if n_atual % 2 == 1:
        soma += n_atual
    n_atual, n_anterior = n_atual + n_anterior, n_atual

print(f"soma: {soma}")
