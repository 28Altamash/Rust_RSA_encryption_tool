# Rust_RSA_encryption_tool
Advanced Scripting 

Project guide 

Submitted by : Altamash Karlekar 

Student I’d: A00281563 

 

RSA Encryption Tool 

This is a simple program written in Rust that performs RSA encryption and decryption operations. RSA is a widely used encryption algorithm for securing data. This tool provides three main functionalities: generating an RSA key pair, encrypting a file, and decrypting a file. Let's break down what each part of the code does in simpler terms. 

I chose to develop the program with ‘OpenSSL’ rather than RSA is because the "OpenSSL" library provides a broader set of cryptographic functionalities beyond RSA, making it a versatile choice for various cryptographic tasks. 

One advantage of using OpenSSL for RSA operations is its maturity and wide adoption. OpenSSL has been around for a long time and is well-tested and trusted by the security community. It has a large user base and is maintained and updated to address security vulnerabilities promptly. This makes it a good choice for cryptographic operations in Rust or any other programming language. 

 

Dependencies 

    The program relies on two external libraries: openssl and clap. 

    openssl is used for RSA encryption and decryption operations. 

    clap helps with command-line argument parsing but isn't actively used in this code. 

Generating an RSA Key Pair 

    The generate_and_save_key_pair function generates an RSA key pair, which consists of a public key and a private key. 

    It saves the generated keys in two separate files: public_key.pem and private_key.pem. 

Encrypting a File 

    The encrypt_file function takes an input file, encrypts its contents using the provided public key, and saves the encrypted data to an output file. 

    It uses the RSA encryption algorithm with padding PKCS1. 

Decrypting a File 

    The decrypt_file function takes an input file containing encrypted data, decrypts it using the private key, and saves the decrypted data to an output file. 

    It also uses the RSA decryption algorithm with PKCS1 padding. 

Main Function 

    The main function is the entry point of the program. 

    It presents a menu to the user with three options: 

    Generate an RSA key pair. 

    Encrypt a file. 

    Decrypt a file. 

    It reads the user's choice and performs the selected operation. 

Overall Package and Dependencies 

    The program uses the Rust programming language. 

    It relies on external libraries (openssl and clap) to simplify RSA operations and command-line input. 

 

Code and Dependencies 

 

 

 

 

 

 

 

 

 

 

 

 Problems faced:  

Issues with the file references in the OpenSSL library 

 

I needed to ensure that the file paths and references to files are correctly handled in the above Rust code. Based on the previous code and error messages, after a bit of research and with the error messages we came to a conclusion that we were encountering problems with reading and writing files. 

 

    Ensuring the file path’s are correct: 

let private_key_file = "/home/altamash/private&public_pem/private_key"; 
	let public_key_file = "/home/altamash/private&public_pem/public_key"; 

    Proper Data Handling: 

// Encrypt data using the public key 
	let ciphertext = public_key.public_encrypt(&data, Padding::PKCS1)?; 

// Decrypt data using the private key 
	let plaintext = private_key.private_decrypt(&ciphertext, Padding::PKCS1)?; 
 

Private.pem (Private Key): 
 

-----BEGIN RSA PRIVATE KEY----- 

MIIEpAIBAAKCAQEAzBsj6E8lUy8sXb4liiQSKF3usodYqL4W6XZ9P4GjgET6ZC8S 

wGcgeUPEMLbU9+qJflwDn7z3e6u5CC+kTTN+/IV5eVWJq6m3NbHw8okl7EJXSS+B 

5M5zsn1v+G7uE2B1Av/tQcNE5M7YHNtCJY1nHPflzAM/zhK7EdY5uGzQ2ZILqFhn 

UEtZzvktMScnp+CNFmRpjNmL9eykC+MnErPSE7NvX4TEhiHgAlf7OMAX6Iw9PLKW 

7LJY5tbUn9jOv8MSXQENIyC6hDaC/dogNN0D0yw0xTV3iD92P27d2jQ8QZZAtJCo 

QdIc8oj9wRHG9tNobKnSD1dznlvdbkiyChdlywIDAQABAoIBAQC1JhIiaP6CFDyi 

bD8SOgX5t3x7W/CSQQ0FtIR+AI61ReG3zKtxXnht78aLu47zcrVp9xMw9sVpUuj2 

NlV9YyWc/J5s0NuKQAlk23ZXI8JKnlDvdxWVbW/33iFZzwhzNou9kxKRsTsgC9Pl 

7u1lP4ZfGDE8RzWbK5/7WRvM8NJNfuDPsIjgWvo8b2z3lGh0CVGMRPFCCYF5gtSR 

uCOcSNp+t43mckimI6Nfs5X64KUM2rt8ELoC1L2QnODS+egqWvxRj0i2J7+K7dp5 

5RcdwmNCTj0e4ojFjbSlni5gC6N58dmS6WQV3n9ZKSDN8I1zkg8NmGq8nnSfLaYF 

yjFqYN5JAoGBAOdEE9N7pL8N3odJL6jE0EOK6d5Dv2VU6dXs3EEVgvoPEVKK/+5I 

kysFLeFE+jwOFtH1zwygMoxGgF8DOmEf56JtCJOKl6IcqBlz2FbQwR3UDYZqqXM1 

gYXtW0bbCKpY7G3U2RpyAL/s0RV4z1lNRdMn/52/VrpkLrFgfu4/z1JDAoGBAOzq 

T93yv4S4iN8u3VCezllZiAIeED88z2swYYWzC8VwEecR4yJyKpBQi9PBaBvmuytv 

NbqTpZm7/Yv/rQK0C7Xr2F6U4ZUQJ5a4g8ML6QIKXbUmHfJgjSHjI0UO4uyRpNpY 

0MzTO/egbQJ4GoA6zU9kgkL9URJtXp4CieKhv3P9mMqaAGleT9Jrse66BHCc6SxI 

TdI9lcCdZHCTvWmUSg2CyJpa7SXq/i6bpn/kboEMI83Q7Elw0AGn94ToI75WU7N9 

IuVZdU0CgYEAqJZ78VfKuYV3Hc9oNnFtnbgMxmyF3LQt6/uSjD8vaUkJ9uDEhzK 

eYP6IlZUPoxF3P5ghCBhjLz5YdLxKB3SJiVAtASZW2W+VEd7ACJcHjNqz8MSerZv 

66kjJXO0IENlQ7vS/rbWULdFesMqBRg/1PwZSKmPtX/hBwpti6ghULMCgYEAzRne 

Kepv5iY0GIEbzgy5t98pF6kncKcA96CVaFmLyjYkgUydmq6Mh/jOmN6MY/y4d9K7 

Odn6A4iLg1NQq/L2nMxlxW7Ll+eb5tkcEf3Y4LsN/FEkEnNHiFWAgNVwLuMZteJH 

uZQsl5qesXrAtIfvOweB0sX4JWJhW4LTKsZIDWMCgYEAuI9gz4rCr+fn2Drq3YFZ 

RZTIRzCNi4OJ8yn2QPtSDJl0FZ7BRmEJ4ZJyhlvzS3t6K3M4zqFQ64YS7/kz80s/ 

NbdowFAJylyFR6uof1sC+9Ig3gJ9cUsGqU8jE4XLJAtwH1DTR0b0EVpVrE6fuwEd 

hRD3/u5b5/+aCAxl4DX8xJkCgYB6aOwD09L24sPTpNCfK07N5/WYiZ4GivzHPPMk 

f6oA2f9bRzszY/kbr+2EzGJzN2MSW1EaTnoBdUpR6U9n7HeWpDsnC3j5XEBbZWG 

 

 

File Encryption, Decryption and CLI Usability- 

 

 

 

 
