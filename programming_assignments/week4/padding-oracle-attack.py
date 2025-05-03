#!/usr/local/bin/python3

# This script implements a padding oracle attack against a target server.
# Non optimized and not very efficient, but it works.

import urllib3
import sys

TARGET = 'http://crypto-class.appspot.com/po?er='

class PaddingOracle(object):
    def query(self, q):
        target = TARGET + q    # Create query URL
        req = urllib3.request("GET", target)         # Send HTTP request to server
        if req.status == 404:
            return True
        else:
            return False

def guessByte(index, blockSize, prevBlock, thisBlock, plainBlock):
    if index == -1:
        return True

    guessBlock = prevBlock.copy()
    padNum = blockSize - index

    # xor with the padding bytes based on PKCS#5 padding
    for i in range(padNum):
        guessBlock[blockSize - 1 - i] ^= padNum

    for g in range(256):
        guessBlock[index] ^= g
        newCiphertext = guessBlock + thisBlock
        if po.query(newCiphertext.hex()):
            print("Guessing byte %d: %02x" % (index, g))
            plainBlock.insert(0, g)
            backup = prevBlock[index]
            prevBlock[index] ^= g

            if guessByte(index - 1, blockSize, prevBlock, thisBlock, plainBlock):
                return True
            else:
                prevBlock[index] = backup
                plainBlock.pop(0)
        guessBlock[index] ^= g

    return False

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: %s <ciphertext_hex_string>" % sys.argv[0])
        sys.exit(1)

    chipertext = bytearray.fromhex(sys.argv[1])  # Convert hex string to binary
    blockSize = 16
    cipherblocks = [bytearray(chipertext[i:i + blockSize]) for i in range(0, len(chipertext), blockSize)]  # Split ciphertext into blocks

    po = PaddingOracle()

    plainText = bytearray()

    print(len(cipherblocks))
    for i in reversed(range(len(cipherblocks))):
        # skip the first block: it is the IV
        if i == 0:
            continue

        prevBlock = cipherblocks[i - 1].copy()
        thisBlock = cipherblocks[i].copy()
        plainBlock = bytearray()

        if guessByte(blockSize - 1, blockSize, prevBlock, thisBlock, plainBlock):
            print(plainBlock)
        else:
            print("Failed to decrypt block %d" % i)
