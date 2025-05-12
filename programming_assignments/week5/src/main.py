#!/usr/local/bin/python3

from gmpy2 import mpz, powmod, mod, invert
from concurrent.futures import ThreadPoolExecutor, as_completed, wait

import json
import sys

def build_left_hash_table(inputs, left_hash_table, start, end):
    B, h, p, q = inputs

    # Prepare the left hash table - h/g^x1 as key and x1 as value
    for x1 in range(start, end):
        left_hash_table[mod(h * invert(powmod(g, x1, p), p), p)] = x1

def meet_in_middle_attack(inputs, left_hash_table, start, end):
    B, h, p, q = inputs

    for x0 in range(start, end):
        # Calculate the right hash value
        right_hash = powmod(g, B * x0, p)

        # Check if the right hash value exists in the left hash table
        if right_hash in left_hash_table:
            x1 = left_hash_table[right_hash]
            return (x0, x1)

    return None

if __name__ == "__main__":
    B = pow(mpz(2), 20)

    # Check if the script is run with the correct number of arguments
    if len(sys.argv) != 3:
        print("Usage: python3 main.py <input_file> <num_threads>")
        sys.exit(1)

    # Read the input file name from command line arguments
    input_file = sys.argv[1]

    # Read the input file
    input_data = {}
    with open(input_file) as f:
        input_data = json.load(f)

    p = mpz(input_data["p"])
    h = mpz(input_data["h"])
    g = mpz(input_data["g"])

    inputs = (B, h, p, g)

    left_hash_table = {}

    num_threads = sys.argv[2]
    build_left_ht_futures = []

    # Build the left hash table in parallel
    with ThreadPoolExecutor(max_workers=num_threads) as executor:
        partitionSize = (B // num_threads)
        for i in range(num_threads):
            start = i * partitionSize
            end = (i + 1) * partitionSize
            build_left_ht_futures.append(executor.submit(build_left_hash_table, inputs, left_hash_table, start, end))

    # Wait for all threads to finish building the left hash table
    wait(build_left_ht_futures)

    futures = []

    # Lookup in parallel
    with ThreadPoolExecutor(max_workers=num_threads) as executor:
        partitionSize = (B // num_threads)
        for i in range(num_threads):
            start = i * partitionSize
            end = (i + 1) * partitionSize
            futures.append(executor.submit(meet_in_middle_attack, inputs, left_hash_table, start, end))

    # Wait for the solution
    for future in as_completed(futures):
        if future.result() is not None:
            x0, x1 = future.result()
            print(f"Found solution: x0 = {x0}, x1 = {x1}, x = {x0 * B + x1}")
            break  # Exit once we find a solution

    # Shutdown the executor (forces remaining threads to exit)
    executor.shutdown(wait=False)
