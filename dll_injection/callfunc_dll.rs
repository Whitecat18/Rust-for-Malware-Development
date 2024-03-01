use std::ffi::CString;
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA};

const PLU: &str = "[+]";

// Some shit i tried . dont laugh at me @_@
// fn calling(handle:*mut HINSTANCE__ ,name: &str) -> *mut __some_function{
//     let name_cstr = Cstring::new(name).unwrap();
//     let name_cstr_ptr = unsafe {
//         GetProcAddress(handle, name_cstr.as_ptr());   
//     };
//     return name_cstr_ptr
// }

fn main(){
    let dll_path = "hook.dll";
    unsafe{
        let dll_path_cstr = CString::new(dll_path).expect("Failed CString");
        let dll_path_ptr = dll_path_cstr.as_ptr();
        let handle = LoadLibraryA(dll_path_ptr);
        if handle.is_null(){
            println!("Failed to load DLL");
            return;
        }

        let name = CString::new("msg_frm_vx").unwrap();
        let name_ptr = GetProcAddress(handle, name.as_ptr());

        let name1 = CString::new("msg_frm_smukx").unwrap();
        let name_ptr1 = GetProcAddress(handle, name1.as_ptr());


        if name_ptr.is_null(){
            println!("{} Failed to get function Address for vx",PLU);
            FreeLibrary(handle);
            return;
        }

        if name_ptr1.is_null(){
            println!("{} Failed to get function Address for smukx",PLU);
            FreeLibrary(handle);
            return;
        }
                        //unsafe
        type MyFunction = extern "stdcall" fn();

        let my_func: MyFunction = std::mem::transmute(name_ptr);
        let my_func1: MyFunction = std::mem::transmute(name_ptr1);
        //exec the func
        my_func();
        my_func1();

        FreeLibrary(handle); 
    }
}
