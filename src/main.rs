mod chacha20;
use chacha20::ChaCha20;
use std::io;

fn main() -> io::Result<()> {
    let mut chacha = ChaCha20::new_default();
    chacha.encrypt_file("files\\input.txt", "files\\output.txt")?; //encrypt the file and output the encrypted string to output.txt
    chacha.set_counter(0); //reset the counter
    chacha.decrypt_file("files\\output.txt", "files\\decrypted.txt")?; //decrypt the file and output the decrypted string to decrypted.txt
    Ok(())
}