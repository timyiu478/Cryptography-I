## Hints

### Q1

```
>>> a = 2**128
>>> b = 10**42
>>> a > b
False
>>> b < a
False
>>> print(a)
340282366920938463463374607431768211456
>>> print(b)
1000000000000000000000000000000000000000000
>>> b > a
True
>>> c = 10**36
>>> print(c)
1000000000000000000000000000000000000
```

### Q2

```
>>> keySpace / searchPerYear / 10**9
2158.0566141612026
```

### Q3

- The definition of secure PRF.
- think which one is NOT secure is easier.

### Q4

1. runs through the 2 round festial network
2. you will find that the ciphertext L1 of F2 is the one's complement of the ciphertext L1 of F2

### Q5

C'0 = F(k, F(k, n = C0) xor M1) = F(k, C1 xor C1 xor C0) = F(k, C0) = C1

### Q6

Each plaintext block only depends on 1 ciphertext block for decryption.

### Q7

The Init Vector is in the first cipher block.

### Q8

- the plaintext size can't larger than the ciphertext.
- CBC mode implies the IV is in the encrypted packet ciphertext, so plaintext < 128 bytes.
- 1 AES S-Box requires 16 bits. 

### Q9

F(k, 0110) = k0 xor k2 xor k3
F(k, 0101) = k0 xor k2 xor k4
F(k, 1110) = k0 xor k1 xor k2 xor k3

F(k, 1101) = F(k, 0110) xor F(k, 1110) xor F(k, 0101)
