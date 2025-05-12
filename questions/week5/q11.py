
p = 13

for g in range(2, p):
    order = [1]
    for i in range(1, p):
        if (g ** i) % p != 1:
            order.append(g ** i % p)
        else:
            break
    if len(order) == (p - 1):
        print(g, order)
