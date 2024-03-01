use std::{ffi::CString, ptr::null_mut};
use winapi::um::winuser::{MessageBoxA, MB_OK};

/*
    #[no_mangle] Short Explain
    - no_mangle is used to interoperate with C code meaning we can call 
        these functions directly from C without any naming conflicts.
    - It can also used to keep its original name during the compile time.
*/
#[no_mangle]
pub extern "stdcall" fn msg_frm_vx(){
    let msg = CString::new("DLL's are awesome ! Especially Exec in Rust").expect("Failed");
    let cap = CString::new("Message From Vx-Underground").expect("Error cap");
    unsafe{
        MessageBoxA(null_mut(), msg.as_ptr(), cap.as_ptr(), MB_OK);
    }
}

// stdcall in C
#[no_mangle]
pub extern "system" fn msg_frm_smukx(){
    let msg = CString::new("Custom DLL's are always Cool. Bye").expect("Failed");
    let cap = CString::new("Message From SMukx").expect("Error cap");
    unsafe{
        MessageBoxA(null_mut(), msg.as_ptr(), cap.as_ptr(), MB_OK);
    }
}
