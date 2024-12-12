use rand::Rng;
use std::fs;
use std::io;

//Represents the ChaCha20 cipher with state and counter
pub struct ChaCha20{
    state: [u32; 16], // ChaCha's 16-word state
    counter: u32,
}


impl ChaCha20{
    //This initializes a a new instance of ChaCha20 with a given 256-bit key and 96-bit nonce
    pub fn new(key: &[u8; 32], nonce: &[u8; 12]) -> Self {
        let mut state = [0u32; 16];

        //Constants for ChaCha20 to initialize the first four words of the state matrix
        state[0] = 0x61707865; // "expa"
        state[1] = 0x3320646e; // "nd 3"
        state[2] = 0x79622d32; // "2-by"
        state[3] = 0x6b206574; // "te k"
    
        // Load the key into the state
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                key[i * 4],
                key[i * 4 + 1],
                key[i * 4 + 2],
                key[i * 4 + 3],
            ]);
        }
    
        state[12] = 0; // Counter starts at 0
    
        // Load the nonce into the state
        for i in 0..3 {
            state[13 + i] = u32::from_le_bytes([
                nonce[i * 4],
                nonce[i * 4 + 1],
                nonce[i * 4 + 2],
                nonce[i * 4 + 3],
            ]);
        }
    
        Self {
            state,
    
            counter: 0, // initial counter value is 0
        }
    }
    // Creates a ChaCha20 instance with a random key and nonce
    pub fn new_default() -> Self {
           let mut random = rand::thread_rng();
            let mut key = [0u8; 32]; // Generates a random 256-bit key
            let mut nonce = [0u8; 12]; // //Generate random 96-bit nonce
            random.fill(&mut key);
            random.fill(&mut nonce);
            Self::new(&key, &nonce)
        }
    //Performs the ChaCha20 quarter round operation
    fn quarter_round(state: &mut [u32], a: usize, b: usize, c: usize, d: usize) {
            // Add, XOR, and rotate operations as per the ChaCha20 algorithm
            state[a] = state[a].wrapping_add(state[b]);
            state[d] ^= state[a];
            state[d] = state[d].rotate_left(16);
    
            state[c] = state[c].wrapping_add(state[d]);
            state[b] ^= state[c];
            state[b] = state[b].rotate_left(12);
    
            state[a] = state[a].wrapping_add(state[b]);
            state[d] ^= state[a];
            state[d] = state[d].rotate_left(8);
    
            state[c] = state[c].wrapping_add(state[d]);
            state[b] ^= state[c];
            state[b] = state[b].rotate_left(7);
        }
        //produces a 64-byte keystream block by applying 20 rounds of ChaCha20 algorithm
        fn block_function(&mut self) -> [u8; 64] {
            let mut working_state = self.state;

            //Apply 20 rounds (10 column rounds followed by 10 diagonal rounds)
            for _ in 0..10 {
                // Column rounds
                Self::quarter_round(&mut working_state, 0, 4, 8, 12);
                Self::quarter_round(&mut working_state, 1, 5, 9, 13);
                Self::quarter_round(&mut working_state, 2, 6, 10, 14);
                Self::quarter_round(&mut working_state, 3, 7, 11, 15);
    
                // Diagonal rounds
                Self::quarter_round(&mut working_state, 0, 5, 10, 15);
                Self::quarter_round(&mut working_state, 1, 6, 11, 12);
                Self::quarter_round(&mut working_state, 2, 7, 8, 13);
                Self::quarter_round(&mut working_state, 3, 4, 9, 14);
            }

            // Add the original state to working state to finalize the block
            for i in 0..16 {
                working_state[i] = working_state[i].wrapping_add(self.state[i]);
            }
    
            // Convert the workign stated to a 64-byte array 
            let mut output = [0u8; 64];
            for i in 0..16 {
                let bytes = working_state[i].to_le_bytes();
                output[4*i..4*i + 4].copy_from_slice(&bytes);
            }
    
            output // Return the keystream block
        }
        //Encryption function
        pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
            let mut result = Vec::with_capacity(data.len());
            let mut data_chunks = data.chunks(64); // Process the data in 64-byte chunks
        
            for chunk in data_chunks {
                // Generate the next keystream block
                let keystream_block = self.block_function();
                self.counter += 1;
                self.state[12] = self.counter;// Update the counter in the state

                // XOR each byte of the plaintext with the keystream
                for (i, &byte) in chunk.iter().enumerate() {
                    result.push(byte ^ keystream_block[i]);
                }
            }
        
            result// Return the encrypted result
        }
        // Decrpytion function
        pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
            // since encryption and decryption are the same
            self.encrypt(data)
        }

        //Encrypts the contents of a file and writes the result to another file
        pub fn encrypt_file(&mut self, input_path: &str, output_path: &str) -> io::Result<()> {
            // Read file contents
            let data = fs::read(input_path)?;
            // Helps distinguish when each process is done in the terminal
            println!("{:?}",data);
            // Encrypt the file data
            let encrypted = self.encrypt(&data); 
            // Helps distinguish when each process is done in the terminal
            println!("{:?}",encrypted);
            //Writes the encrypted data to output file
            fs::write(output_path, encrypted)?;
            Ok(())
        }
        
        //Decrypts the contents of a file and writes the result to another file
        pub fn decrypt_file(&mut self, input_path: &str, output_path: &str) -> io::Result<()> {
            // Read file contents
            let data = fs::read(input_path)?;
            // Helps distinguish when each process is done in the terminal
            println!("{:?}",data);
            // Encrypt the file data
            let decrypted = self.decrypt(&data);
            // Helps distinguish when each process is done in the terminal
            println!("{:?}",decrypted);
             //Writes the encrypted data to output file
            fs::write(output_path, decrypted)?;
            Ok(())
        }
        //A setter function to manually set the counter and update the state to ensure integrity of algorithm
        pub fn set_counter(&mut self, counter: u32) {
            self.counter = counter;
            self.state[12] = counter;
        }

}

