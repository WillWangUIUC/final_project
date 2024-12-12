mod chacha20;
use chacha20::ChaCha20;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("{}",1); //make sure the program is running
    let mut chacha = ChaCha20::new_default();
    chacha.encrypt_file("files\\input.txt", "files\\output.txt")?;
    chacha.set_counter(0);
    chacha.decrypt_file("files\\output.txt", "files\\decrypted.txt")?;
    Ok(())
}