#!/usr/bin/python3

left_input_pairs = [
    ["290b6e3a", "d6f491c5"],
    ["5f67abaf", "bbe033c0"],
    ["9d1a4f78", "75e5e3ea"],
    ["7b50baab", "ac343a22"],
]

for input_pair in left_input_pairs:
    byte_input_1 = bytearray.fromhex(input_pair[0])
    byte_input_2 = bytearray.fromhex(input_pair[1])

    if bytes(a ^ b for (a, b) in zip(byte_input_1, byte_input_2)) == bytearray.fromhex("ffffffff"):
        print(input_pair)
