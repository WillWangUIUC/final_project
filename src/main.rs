mod chacha20;
use chacha20::ChaCha20;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    println!(1);
    let mut chacha = ChaCha20::new_default();
    chacha.encrypt_file("files\input.txt", "files\output.txt")?;
    Ok(())
}