use rand::Rng;

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
        //more needed to come
    }
    pub fn new_default()-> Self{
        pub fn new_default() -> Self {
            let mut random = rand::thread_rng();
            let mut key = [0u8; 32];
            let mut nonce = [0u8; 12];
            random.fill(&mut key);
            random.fill(&mut nonce);
            Self::new(&key, &nonce)
        }
    }
}
