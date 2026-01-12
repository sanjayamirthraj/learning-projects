use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;

fn main() {
    loop {
        print!("> ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
                
        //args are just going to be the words after the first word which is the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;

        match command {
            "cd" => {
                let new_path = args.next().unwrap_or("~");
                let root = Path::new(new_path);
                if let Err(err) = std::env::set_current_dir(root) {
                    eprintln!("cd: {}: {}", new_path, err);
                }
            }
            "git commit -m" => {
                let message = input.split("git commit -m ").nth(1).unwrap();
                let mut child_command = Command::new("git").args(["commit", "-m", message]).spawn().unwrap();
                child_command.wait();
            }
            command => {
                let mut child_command = Command::new(command).args(args).spawn().unwrap();
                child_command.wait();  
            }
        }

       
    }


}
