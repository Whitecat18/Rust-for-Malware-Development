struct Reply{
    reget: i32,
}

impl Reply{
    pub fn reply(&self){
        if self.reget == IDCANCEL{
            println!("User gave cancel");
        }
        if self.reget == IDTRYAGAIN{
            println!("Try again button");
        }
        else{
            println!("User gave continue!")
        }
    }
}

use winapi::um::winuser::*;
use winapi::um::winuser::{MB_CANCELTRYCONTINUE, MB_ICONWARNING , MB_DEFBUTTON2};

fn main(){
    let text = "You need learn as fk as you can..\0";
    let title = "Message from Smukx\0";
    
    // let mut count = 0;

    let status;
        unsafe{
            status = MessageBoxA(std::ptr::null_mut(),
                text.as_bytes().as_ptr() as *const i8 ,
                title.as_bytes().as_ptr() as *const i8,
                MB_CANCELTRYCONTINUE | MB_ICONWARNING | MB_DEFBUTTON2);
            }

    let out = Reply{reget: status};
    out.reply();
}