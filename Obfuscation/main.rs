use std::{env::Args, net::Ipv4Addr};

// IpV4 Obfuscation ...! 
fn obfuscate_ipv4(shellcode: &mut Vec<u8>){
    if shellcode.len() % 4 != 0 {
        while shellcode.len() % 4 != 0{
            shellcode.push(0);
        }
    }
    println!("shellcode : [");
    for ip_addr in shellcode.chunks(4){
        let ip = format!("{}.{}.{}.{}", ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]);
        print!("{:?}", ip);
    }
    println!("]");

}

fn deobfuscate_ipv4(ipv4_addr: Vec<&str>) -> Result<Vec<u8>, ()>{
    let mut debug: Vec<u8> = Vec::with_capacity(ipv4_addr.len() * 4);

    for ip in ipv4_addr{
        match ip.parse::<Ipv4Addr>(){
            Ok(ip_addr) => debug.extend_from_slice(&ip_addr.octets()),
            Err(_) => return Err(()),
        }
    }
    Ok(debug)
}

// add utils file . ! 
fn main(){
    let args = utils::Args::parse();
    // Will be coding at remove area ;) 
}
