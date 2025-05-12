import math

def are_coprime(a, b):
    return math.gcd(a, b) == 1  # Check if GCD is 1

count = 0

for i in range(1, 35):
    if are_coprime(i, 35):
        count += 1

print(count)
