/* 
Malware Basics: Allocating at Windows Memory via Rust Functions and Windows API'S ! 

For more codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
By: @5mukx

*/

// MANUAL MEMORY ALLOCATION WITHOUT [winapi] aka WINDOWS API.

/* 

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::ffi::CString;

use std::ptr::copy_nonoverlapping;

fn main(){
    let size = 100;

    let layout = Layout::from_size_align(size, std::mem::align_of::<u8>()).unwrap();
    
    // Allocate memory with global Allocator 
    let p_addr = unsafe { alloc(layout)};

    unsafe{
        if p_addr.is_null(){
            // filling the allocated memory with 0 .
            ptr::write_bytes(p_addr, 0, size);
            // Using CString, An C-compatible, nul-terminated string with no nul bytes in the middle.

            let string = CString::new("Maldev hits diffrerent").expect("Error while creating cstring");
            
            // copy_nonoverlapping is semantically equivalent to C's memcpy but with the argument order swapped
                copy_nonoverlapping(string.as_ptr(), p_addr as *mut i8, string.as_bytes().len());

                let content = std::slice::from_raw_parts(p_addr, string.as_bytes().len());

                println!("[+] Memory Content: {:?}",content);
                
                println!("[+] Deallocating Mem contnet");
                dealloc(p_addr, layout);
            } else {
                println!("[-] Failed to allocate memory");
            }
        }
}

*/

// MEMORY ALLOCATION USING [winapi]

/*
Make sure you have include these dependencies on Cargo.toml file !

[dependencies]
winapi = { version = "0.3", features = ["minwindef", "winbase"] }
*/



use winapi::um::heapapi::{GetProcessHeap, HeapAlloc, HeapFree};
use std::slice::from_raw_parts;
fn main(){
    unsafe{
        let heap = GetProcessHeap();
        if heap.is_null(){
            println!("[-] Failed to get process heap");
            return
        }
        
        // https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc
        // 0x00000008 -> /. similar to winnt::HEAP_ZERO_MEMORY; 
        let p_address = HeapAlloc(heap, 0x00000008, 100);

        if p_address.is_null(){
            println!("[-] Failed to allocate memory on the heap");
            return
        }

        println!("[+] Base Address of Allocated memory: {:#?}",p_address);

        let string = "Maldev hits different".as_ptr() as *const u8;


        std::ptr::copy_nonoverlapping(string , p_address as *mut u8, 100);

        let content = from_raw_parts(p_address as *const u8, 100);
        
        println!("[+] Memory content: {:?}", content);

        
        HeapFree(heap, 0, p_address);

        println!("[+] Freed Allocated memory !");
    }
}
