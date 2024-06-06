// Just an sample Example of using using extern to Run MessageBoxA ..!
// Create an Function with the help of Microsoft Documentation !

use std::ptr::null_mut;

extern "system" {
    fn MessageBoxA(
        hWnd: *mut std::ffi::c_void,
        lpText: *const u8,
        lpCaption: *const u8,
        uType: u32,
    ) -> i32;
}

fn main() {
    let text = b"Hello Nerds..\0".as_ptr();
    let caption = b"Meow_Meow\0".as_ptr();

    unsafe {
        MessageBoxA(null_mut(), text, caption, 0x00000000);
    }
}
