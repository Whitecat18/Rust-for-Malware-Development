
use std::ffi::CString;
use std::ptr::null_mut;

use winapi::shared::minwindef::HINSTANCE;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::um::winuser::{MessageBoxA,MB_OK,MB_ICONEXCLAMATION,MB_ICONWARNING};
use winapi::um::libloaderapi::{GetModuleHandleW, FreeLibraryAndExitThread};


#[no_mangle]
#[allow(unused_variables)]
extern "C" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ()) -> bool 
{
        match call_reason{
            DLL_PROCESS_ATTACH => attach(),
            DLL_PROCESS_DETACH => detach(),
            _ => ()
        }
        true
}
#[no_mangle]
fn attach(){
    unsafe{
        let text: CString = CString::new("Custom DLL's are fking awesome !").expect("Failed to convert CString");
        let caption = CString::new("By nerdy smukx").expect("Failed to convert into cstring");
        // let text = "Custom DLL's are fking awesome !";
        // let caption ="Msg by vx_unknown";
        MessageBoxA(null_mut(), text.as_ptr() as *const i8, caption.as_ptr() as *const i8, MB_OK | MB_ICONEXCLAMATION);
        // JUST CALLING THE DETACH FUNCTION !
        detach()
    }
}

#[no_mangle]
fn detach(){
    unsafe{
        let text = CString::new("My moto is to leave malware resources for free ").expect("Failed to convert CString");
        let caption = CString::new("Msg by vx_underground").expect("Failed to convert into cstring");
        let instance = GetModuleHandleW(std::ptr::null());
        MessageBoxA(null_mut(), text.as_ptr() as *const i8, caption.as_ptr() as *const i8, MB_OK | MB_ICONWARNING);
        
        FreeLibraryAndExitThread(instance, 0);
    }
}
