extern crate clap;
use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("Encryption CLI")
        .version("1.0")
        .author("Your Name")
        .about("A user-friendly CLI for encryption operations")
        .arg(
            Arg::with_name("option")
                .short("o")
                .long("option")
                .value_name("OPTION")
                .help("Choose an option: 1 for Generate, 2 for Encrypt, 3 for Decrypt, 4 for Quit")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let option = matches
        .value_of("option")
        .unwrap()
        .parse::<u32>()
        .expect("Invalid option. Please choose a valid option (1-4).");

    match option {
        1 => generate_key_pairs(),
        2 => encrypt(),
        3 => decrypt(),
        4 => quit(),
        _ => {
            eprintln!("Invalid option. Please choose a valid option (1-4).");
            process::exit(1);
        }
    }
}

fn generate_key_pairs() {
    // Implement key pair generation logic here
    println!("Generating key pairs...");
}

fn encrypt() {
    // Implement encryption logic here
    println!("Encrypting...");
}

fn decrypt() {
    // Implement decryption logic here
    println!("Decrypting...");
}

fn quit() {
    println!("Quitting.");
    process::exit(0);
}
