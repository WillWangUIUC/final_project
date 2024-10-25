# final_project

Proposal：Secret Spy Encryption Program

Group Name: Secure Bytes

Group member names and NetIDs
Zihao Wang zihaow6
Hemanth Itte hitte

Project Introduction:
We are building a ChaCha20 encryption program. A spy will send their secret information into this program to be encrypted. When Headquarters receives this encrupted data, they will use the program to decrpyt the data and save the world. Our goals for this project is to build a ChaCha20 encrytion program that will take in a file with data and encrypt using the ChaCha20 algorithm, we also want to be able to decrypt the data at a later point and print it out without any loss of data. We chose this project because we both love cybersecurity topics and found encryption and decrytion a very interesting topic, especially the complex math algorithms that certain programs utalize to encrypt data very securly. After some time researching online we found ChaCha20. Most encryption programs are reliant on a fixed-size blocks of data, but ChaCha20 is not reliant and is very flexible. We also found the qaurter-rounds very interesting and so decided to create a ChaCha20 program. We chose for it to be a spy program because ChaCha20 is commonly used in file encrpytion and we wanted a fun story to go along with our program.

1. ChaCha20 takes in a initial 4x4 state matrix looks like this:
	[ constants (4 words) | key (8 words) | counter (1 word) | nonce (3 words) ]
	2. Then we'll perform a Quarter-Round Function:
		a. A quarter-round takes four 32-bit words and performs a sequence of additions, XORs, and rotations:
			i. a += b; d ^= a; d <<<= 16
			ii. c += d; b ^= c; b <<<= 12
			iii. a += b; d ^= a; d <<<= 8
			iv. c += d; b ^= c; b <<<= 7
		b. This mixes the bits in a way that provides diffusion (where changes in one part of the input affect the entire output).
	3. We'll implement the block function that takes the initial state matrix and applies 20 rounds of transformations. Each round consists of two steps:
		a. Column Round: Applies the quarter-round function to four columns of the state matrix.
		b. Diagonal Round: Applies the quarter-round function to diagonals of the state matrix.
		c.  These operations mix the matrix contents thoroughly, ensuring that the output is pseudo-random and secure.
	4. Producing the Keystream:
		a.The final keystream block is obtained by adding the transformed state matrix to the original state matrix (element-wise addition). This keystream block is 64 bytes long (16 words x 4 bytes per word).
	

Check point1 (11/8): basic functionality
Check point2 (11/22): improve functionality
Final phase (12/11): decryption algorithm

Potential challenges:
Correctly understanding the encryption mechanism 
How to improve the performance of the algrithm
Potentially creating a channel to communicate files and keys
Learning how to read data from file and output data to a file


