#!/usr/local/bin/python3

import sys
import hashlib

BLOCK_SIZE = 1024


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: ./main.py <filename>")
        sys.exit(1)

    filename = sys.argv[1]

    blocks = []

    with open(filename, "rb") as file:
        while block := file.read(BLOCK_SIZE):
            blocks.append(block)

    prevHash = b''

    for block in reversed(blocks):
        prevHash = hashlib.sha256(block + prevHash).digest()

    print(prevHash.hex())
