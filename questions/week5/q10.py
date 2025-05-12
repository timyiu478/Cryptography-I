g = 2

order = [1]

for i in range(1, 35):
    if (g ** i) % 35 != 1:
        order.append(g ** i)
    else:
        break

print(order)
print(len(order))
