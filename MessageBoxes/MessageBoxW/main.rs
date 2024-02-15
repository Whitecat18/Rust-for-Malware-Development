struct Reply;

impl Reply{
    pub fn output(res: i32){
        match res{
            IDYES => println!("Ohh i see.. Okk lets create more malware"),
            _ => println!("Oops .. so Lets learn it"),
        }
    }
}
use winapi::um::winuser::{
    MessageBoxW, 
    IDYES, 
    MB_ICONASTERISK ,
    MB_YESNO};

fn main(){
    let text = "Are you a maldev\0".encode_utf16().collect::<Vec<_>>();
    // OR you can mut these vaiables and push Null byte on these vecs !.
    let title = "Sweety Box\0".encode_utf16().collect::<Vec<_>>();

    let reply = unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            MB_YESNO | MB_ICONASTERISK 
        )
    };
    Reply::output(reply);

}
