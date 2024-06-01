/*
    Shellcode Obfuscation and Deobfuscation Technique
    Wonderful Resource By Maldev Academy
    @5mukx
*/

use std::fs::File;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{Ipv4Addr, Ipv6Addr};
use uuid::Uuid;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[clap(about = "Shellcode Obfuscation and Deobfuscation Technique")]
pub struct Args {
    #[clap(short, long, required = true, help = "Enter shellcode file")]
    pub file: String,

    #[clap(short, long, required = true, help = "Enter obfuscation type")]
    pub technique: Obfuscation,

    #[clap(short, long, required = true, help = "Specify whether to obfuscate or deobfuscate")]
    pub operation: Operation,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Obfuscation {
    IPV4,
    IPV6,
    MAC,
    UUID,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Operation {
    Obfuscate,
    Deobfuscate,
}

// IPv4 Obfuscation
fn obfuscate_ipv4(shellcode: &mut Vec<u8>, output_file: &str) {
    if shellcode.len() % 4 != 0 {
        while shellcode.len() % 4 != 0 {
            shellcode.push(0);
        }
    }
    let mut file = File::create(output_file).expect("Could not create output file");
    for ip_addr in shellcode.chunks(4) {
        let ip = format!("{}.{}.{}.{}", ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]);
        writeln!(file, "{}", ip).expect("Could not write to output file");
    }
}

fn deobfuscate_ipv4(file: &str) -> Result<Vec<u8>, ()> {
    let file = File::open(file).map_err(|_| ())?;
    let reader = BufReader::new(file);
    let mut debug: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let ip = line.unwrap().trim().to_string();
        match ip.parse::<Ipv4Addr>() {
            Ok(ip_addr) => debug.extend_from_slice(&ip_addr.octets()),
            Err(_) => return Err(()),
        }
    }
    Ok(debug)
}

// IPv6 Obfuscation
fn obfuscate_ipv6(shellcode: &mut Vec<u8>, output_file: &str) {
    if shellcode.len() % 16 != 0 {
        while shellcode.len() % 16 != 0 {
            shellcode.push(0);
        }
    }
    let mut file = File::create(output_file).expect("Could not create output file");
    for chunk in shellcode.chunks(16) {
        let ip = format!(
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15]
        );
        writeln!(file, "{}", ip).expect("Could not write to output file");
    }
}

fn deobfuscate_ipv6(file: &str) -> Result<Vec<u8>, ()> {
    let file = File::open(file).map_err(|_| ())?;
    let reader = BufReader::new(file);
    let mut deobfuscated_ips: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let ip = line.unwrap().trim().to_string();
        match ip.parse::<Ipv6Addr>() {
            Ok(ip_addr) => {
                for segment in ip_addr.segments() {
                    deobfuscated_ips.extend_from_slice(&segment.to_be_bytes());
                }
            }
            Err(_) => return Err(()),
        }
    }

    Ok(deobfuscated_ips)
}

// MAC Obfuscation
fn obfuscate_mac(shellcode: &mut Vec<u8>, output_file: &str) {
    let mut file = File::create(output_file).expect("Could not create output file");

    let mac_addresses: Vec<String> = shellcode
        .chunks(6)
        .map(|chunk| {
            chunk
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join(":")
        })
        .collect();

    for mac in &mac_addresses {
        writeln!(file, "{}", mac).expect("Could not write to output file");
    }
}

fn deobfuscate_mac(file: &str) -> Result<Vec<u8>, ()> {
    let file = File::open(file).map_err(|_| ())?;
    let reader = BufReader::new(file);
    let mut original_ints: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let mac = line.unwrap().trim().to_string();
        let bytes: Vec<u8> = mac
            .split(':')
            .map(|byte_str| u8::from_str_radix(byte_str, 16).unwrap())
            .collect();
        original_ints.extend_from_slice(&bytes);
    }

    Ok(original_ints)
}

// UUID Obfuscation
fn obfuscate_uuid(shellcode: &mut Vec<u8>, output_file: &str) {
    let mut file = File::create(output_file).expect("Could not create output file");
    let uuids: Vec<Uuid> = shellcode
        .chunks(16)
        .map(|chunk| {
            let mut array = [0; 16];
            for (i, &byte) in chunk.iter().enumerate() {
                array[i] = byte;
            }
            Uuid::from_bytes(array)
        })
        .collect();

    for uuid in &uuids {
        writeln!(file, "{}", uuid).expect("Could not write to output file");
    }
}

fn deobfuscate_uuid(file: &str) -> Result<Vec<u8>, ()> {
    let file = File::open(file).map_err(|_| ())?;
    let reader = BufReader::new(file);
    let mut desofuscated_bytes = Vec::new();

    for line in reader.lines() {
        let uuid_str = line.unwrap().trim().to_string();
        match Uuid::parse_str(&uuid_str) {
            Ok(uuid) => {
                desofuscated_bytes.extend_from_slice(uuid.as_bytes());
            }
            Err(_) => return Err(()),
        }
    }

    Ok(desofuscated_bytes)
}

fn main() -> std::io::Result<()> {
    let argument = Args::parse();
    let file = argument.file;
    let technique = argument.technique;
    let operation = argument.operation;
    let mut shellcode = File::open(&file)?;
    let mut buffer: Vec<u8> = Vec::new();
    shellcode.read_to_end(&mut buffer)?;

    match operation {
        Operation::Obfuscate => {
            match technique {
                Obfuscation::IPV4 => {
                    obfuscate_ipv4(&mut buffer, "obfuscated_ipv4.txt");
                }
                Obfuscation::IPV6 => {
                    obfuscate_ipv6(&mut buffer, "obfuscated_ipv6.txt");
                },
                Obfuscation::MAC => {
                    obfuscate_mac(&mut buffer, "obfuscated_mac.txt");
                }
                Obfuscation::UUID => {
                    obfuscate_uuid(&mut buffer, "obfuscated_uuid.txt");
                }
            }
            println!("Obfuscation successful, output written to file.");
        }
        Operation::Deobfuscate => {
            let result = match technique {
                Obfuscation::IPV4 => deobfuscate_ipv4(&file),
                Obfuscation::IPV6 => deobfuscate_ipv6(&file),
                Obfuscation::MAC => deobfuscate_mac(&file),
                Obfuscation::UUID => deobfuscate_uuid(&file),
            };

            match result {
                Ok(deobfuscated) => {
                    let mut output_file = File::create("deobfuscated_output.bin")?;
                    output_file.write_all(&deobfuscated)?;
                    println!("Deobfuscation successful, output written to deobfuscated_output.bin");
                }
                Err(_) => {
                    eprintln!("Deobfuscation failed");
                }
            }
        }
    };

    Ok(())
}
