extern crate openssl;

use openssl::rsa::{Rsa, Padding};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Usage: {} <encrypt/decrypt> <input_file> <output_file> <public_key_file>", args[0]);
        return;
    }

    let operation = &args[1];
    let input_file_path = &args[2];
    let output_file_path = &args[3];
    let public_key_file_path = &args[4];

    match operation.as_str() {
        "encrypt" => {
            encrypt_file(input_file_path, output_file_path, public_key_file_path);
            println!("File encrypted successfully!");
        }
        "decrypt" => {
            decrypt_file(input_file_path, output_file_path, public_key_file_path);
            println!("File decrypted successfully!");
        }
        _ => {
            println!("Invalid operation. Use 'encrypt' or 'decrypt'.");
        }
    }
}

fn encrypt_file(input_file_path: &str, output_file_path: &str, public_key_file_path: &str) {
    let mut input_file = File::open(input_file_path).expect("Failed to open input file");
    let mut output_file = File::create(output_file_path).expect("Failed to create output file");

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).expect("Failed to read input file");

    let public_key_pem = read_file(public_key_file_path);
    let public_key = Rsa::public_key_from_pem(public_key_pem.as_bytes()).expect("Failed to read public key");

    let encrypted_data = public_key.public_encrypt(input_data, Padding::PKCS1).expect("Encryption failed");

    output_file.write_all(&encrypted_data).expect("Failed to write encrypted data to output file");
}

fn decrypt_file(input_file_path: &str, output_file_path: &str, private_key_file_path: &str) {
    let mut input_file = File::open(input_file_path).expect("Failed to open input file");
    let mut output_file = File::create(output_file_path).expect("Failed to create output file");

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).expect("Failed to read input file");

    let private_key_pem = read_file(private_key_file_path);
    let private_key = Rsa::private_key_from_pem(private_key_pem.as_bytes()).expect("Failed to read private key");

    let decrypted_data = private_key.private_decrypt(input_data, Padding::PKCS1).expect("Decryption failed");

    output_file.write_all(&decrypted_data).expect("Failed to write decrypted data to output file");
}

fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = File::open(path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    content
}
