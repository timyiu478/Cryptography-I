x = 0

z = 13

base = 2

input = 5

while True:
    if  (base ** x) % 13 == input:
        break
    else:
        x += 1

print(f"DLog of {input} of base {base} is {x}")
