/*
  Not completed fully nerds. Just remote backup nerds.
  NOTECODE: Notion: 252458 [notes + poc]
  
*/

use base64::{encode};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn obfuscate_powershell_script(script_path: &str, output_path: &str) -> io::Result<()> {

  let script_content = fs::read_to_string(script_path)?;
    let encoded_script = encode(script_content);
    let obfuscated_script = format!(
        "powershell -encodedCommand {}",
        encoded_script
    );
    let mut output_file = fs::File::create(output_path)?;
    output_file.write_all(obfuscated_script.as_bytes())?;
  
    println!("Obfuscated ps1 file: {}", output_path);
    Ok(())
}

fn main() {
    // Define the input and output paths
    let script_path = "winpeas.ps1";
    let output_path = "winpeasobfus.ps1";

    if !Path::new(script_path).exists() {
        eprintln!("Input script file does not exist: {}", script_path);
        return;
    }
    if let Err(e) = obfuscate_powershell_script(script_path, output_path) {
        eprintln!("Error obfuscating script: {}", e);
    }
}
