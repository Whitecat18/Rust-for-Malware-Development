use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let stream = TcpStream::connect("127.0.0.1:4444").await?;
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut stdin_reader = BufReader::new(stdin);

        loop {
            let mut command = String::new();
            stdin_reader.read_line(&mut command).await.expect("Failed to read from stdin");
            writer.write_all(command.as_bytes()).await.expect("Failed to write to server");
            writer.flush().await.expect("Failed to flush writer");

            if command.trim().starts_with("upload ") {
                let mut file_content = String::new();
                stdin_reader.read_line(&mut file_content).await.expect("Failed to read file content from stdin");
                writer.write_all(file_content.as_bytes()).await.expect("Failed to write file content to server");
                writer.flush().await.expect("Failed to flush writer");
            }
        }
    });

    loop {
        let mut response = String::new();
        let bytes_read = reader.read_line(&mut response).await?;
        if bytes_read == 0 {
            break; 
        }
        print!("{}", response);
    }

    Ok(())
}
