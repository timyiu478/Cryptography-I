## Many Time Pad

Let us see what goes wrong when a stream cipher key is used more than once.  Below are eleven hex-encoded ciphertexts that are the result of encrypting eleven plaintexts with a stream cipher, all with the same stream cipher key. Our goal is to decrypt the last ciphertext, and submit the secret message within it as solution. 

Hint: XOR the ciphertexts together, and consider what happens when a space is XORed with a character in [a-zA-Z].

## Remarks

- The output of the script is not fully correct. You still have to guess some characters.
