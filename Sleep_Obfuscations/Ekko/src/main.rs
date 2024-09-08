/*
    Sleep Obfuscation Ekko.

    Written by 5mukx
    Original Author goes to @C5pider

    Here, I have written two obfuscation methods: 
        one is my own implementation via the 5pidey POC, and the other is a well-known, commonly used method.


*/

mod ekko_common;
// mod ekko_smukx;

fn main(){

    println!("Ekko SLeep Obfuscation in Rust");

    loop{
        ekko_common::ekko(4 * 1000);
        // ekko_smukx::smart_ekko(4000);
    }
}

