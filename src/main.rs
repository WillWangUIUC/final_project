mod chacha20;
use chacha20::ChaCha20;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("{}",1); //make sure the program is running
    let mut chacha = ChaCha20::new_default();
    chacha.encrypt_file("files\\input.txt", "files\\output.txt")?; //encrypt the file and output the encrypted string to output.txt
    chacha.set_counter(0); //reset the counter
    chacha.decrypt_file("files\\output.txt", "files\\decrypted.txt")?; //decrypt the file and output the decrypted string to decrypted.txt
    Ok(())
}