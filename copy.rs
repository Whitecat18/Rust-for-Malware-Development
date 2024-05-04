// Simple Program to look and copy the programs .. !
// @5mukx ..!

use std::io::{self, Error, Write};

use std::fs;
use std::path::Path;
use std::process::Command;
use clipboard::{ClipboardContext, ClipboardProvider};
use walkdir::WalkDir;


fn main(){
    println!("Welcome to the Maldev Mini Shell (MMS) ;)");
    println!("Type 'help' for list of commands, exit to exit the program");

    loop{
        print!("Smx > ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the input");
        let mut parts = input.trim().splitn(2, ' ');

        let command = parts.next().unwrap();
        let argument = parts.next().unwrap_or("");

        match command{
            "help" =>{
                    println!("Avaiable Commands:");
                    println!("  help                -   Display the help message");
                    println!("  list                -   List all the files in the current directory");
                    println!("  clear               -   Clear the Screen");
                    println!("  look <filename>     -   Look at the content of the Program");
                    println!("  copy <filename>     -   Copy the content of the Program");
                    println!("  exit                -   Exit the Program");
                }
            
                "look" => {
                    look_file_content(argument);
                }
                "copy" => {
                    copy_file_to_clipboard(argument);
                }
                "list" => {
                    list_files_tree();
                }
                "clear" => {
                    clear();
                }
                "exit" => {
                    println!("Exiting program...");
                    break;
                }
                _ => {
                    println!("Invalid command. Type 'help' for a list of commands.");
                }
            }
        }
}


fn look_file_content(file_name: &str) {
    if let Some(content) = read_file_content(file_name) {
        println!("Content of '{}':\n{}", file_name, content);
    }
}

fn copy_file_to_clipboard(file_name: &str) {
    if let Some(content) = read_file_content(file_name) {
        match copy_to_clipboard(&content) {
            Ok(_) => println!("Content copied to clipboard successfully!"),
            Err(_) => eprintln!("Error: Failed to copy content to clipboard"),
        }
    }
}

fn read_file_content(file_name: &str) -> Option<String> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Error: Failed to get current directory");
            return None;
        }
    };

    let file_path = current_dir.join(file_name);
    if let Ok(content) = fs::read_to_string(&file_path) {
        Some(content)
    } else {
        eprintln!("Error: Failed to read file '{}'", file_path.display());
        None
    }
}

fn copy_to_clipboard(content: &str) -> Result<(), Error> {
    let mut ctx = ClipboardContext::new().expect("Error");
    let _ = ctx.set_contents(content.to_owned());
    Ok(())
}

fn list_files_tree() {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Error: Failed to get current directory");
            return;
        }
    };

    println!("Files in current directory (tree view):");
    list_files_recursively(&current_dir, 0);
}

fn list_files_recursively(dir: &Path, level: usize) {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy();
        let indentation = "  ".repeat(level);
        if entry.file_type().is_dir() {
            println!("{}{} [Directory]", indentation, file_name);
        } else {
            println!("{}{}", indentation, file_name);
        }
    }
}

fn clear(){
    if cfg!(target_os = "windows"){
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else{
        Command::new("sh").arg("-c").arg("clear").status().unwrap();
    }
}
