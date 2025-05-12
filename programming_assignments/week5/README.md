# Discrete Logarithm Algorithm with meet-in-the-middle attack

Rely on the fact that $x < B^2$ where $B = 2^{20}$.

![](assets/dlog_meet_in_the_middle_attack.png)

Performance Optimisations:

1. build the hash table in parrell by each thread computing $h/g^{x_1}$ for their own $x_1$ range partition.
2. lookup the hash table in parrell by each thread computing $g^{B{x_0}}$ for their own $x_0$ range partition.

# Source Code

See [src](./src)

# Problem

Find $x$ such that $h = g^x \in Z_p$.

![](assets/problem.png)
