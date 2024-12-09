use rand::Rng;
use std::fs;
use std::io;

pub struct ChaCha20{
    state: [u32; 16],
    key: [u8; 32],
    nonce: [u8; 12],
    counter: u32,
}


impl ChaCha20{
    pub fn new(key: &[u8;32], nonce: &[u8;12]) -> Self{
        let mut state = [0u32; 16];
        state[0] = 0x61707865; // "expa"
        state[1] = 0x3320646e; // "nd 3"
        state[2] = 0x79622d32; // "2-by"
        state[3] = 0x6b206574; // "te k"
        for i in 0..8{
            let start = 1*4;
            state[4 + i] = u32::from_le_bytes([
                nonce[start],
                nonce[start+1],
                nonce[start+2],
                nonce[start+3]
            ]);
        }
        state[12] = 0;
        for i in 0..3{
            let start = 1*4;
            state[13 + i] = u32::from_le_bytes([
                nonce[start],
                nonce[start+1],
                nonce[start+2],
                nonce[start+3]
            ]);
        }
        Self{
            state,
            key: *key,
            nonce: *nonce,
            counter: 0,
        }
    }
    pub fn new_default() -> Self {
            let mut random = rand::thread_rng();
            let mut key = [0u8; 32];
            let mut nonce = [0u8; 12];
            random.fill(&mut key);
            random.fill(&mut nonce);
            Self::new(&key, &nonce)
        }
    
    fn quarter_round(state: &mut [u32], a: usize, b: usize, c: usize, d: usize) {
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
    
        fn block_function(&mut self) -> [u8; 64] {
            let mut working_state = self.state;
    
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
    
            for i in 0..16 {
                working_state[i] = working_state[i].wrapping_add(self.state[i]);
            }
    
            // Convert to bytes
            let mut output = [0u8; 64];
            for i in 0..16 {
                let bytes = working_state[i].to_le_bytes();
                output[4*i..4*i + 4].copy_from_slice(&bytes);
            }
    
            output
        }
    
        pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
            let mut result = Vec::with_capacity(data.len());
            let mut offset = 0;
    
            while offset < data.len() {
                self.state[12] = self.counter;
                let keystream = self.block_function();
                let chunk_size = std::cmp::min(64, data.len() - offset);
    
                for i in 0..chunk_size {
                    result.push(data[offset + i] ^ keystream[i]);
                }
    
                offset += 64;
                self.counter = self.counter.wrapping_add(1);
            }
    
            result
        }
    
        pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
            // since encryption and decryption are the same
            self.encrypt(data)
        }
    
        pub fn encrypt_file(&mut self, input_path: &str, output_path: &str) -> io::Result<()> {
            let data = fs::read(input_path)?;
            let encrypted = self.encrypt(&data);
            fs::write(output_path, encrypted)?;
            Ok(())
        }
    
        pub fn decrypt_file(&mut self, input_path: &str, output_path: &str) -> io::Result<()> {
            let data = fs::read(input_path)?;
            let decrypted = self.decrypt(&data);
            fs::write(output_path, decrypted)?;
            Ok(())
        }

}

