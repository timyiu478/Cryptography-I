#!/usr/bin/python3

orginal_msg = "Pay Bob 100$".encode("utf8")
modified_msg = "Pay Bob 500$".encode("utf8")

# prepend 0
while len(orginal_msg) < 256:
    orginal_msg += b'0'
while len(modified_msg) < 256:
    modified_msg += b'0'

iv =  bytearray.fromhex("20814804c1767293b99f1d9cab3bc3e7")
c0 = "ac1e37bfb15599e5f40eef805488281d"

tmp = bytes([b1 ^ b2 for b1, b2 in zip(iv, orginal_msg)])

modified_iv = bytes([b1 ^ b2 for b1, b2 in zip(tmp, modified_msg)])

print(modified_iv.hex(), c0)
