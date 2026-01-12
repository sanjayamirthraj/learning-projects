use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::process::Stdio;

fn main() {
    loop {
        print!("> ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;
        while let Some(command) = commands.next() {
            //args are just going to be the words after the first word which is the command
            let mut parts = command.trim().split_whitespace();
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
                "git" => {
                    if input.starts_with("git commit -m ") {
                        let message = input.split("git commit -m ").nth(1).unwrap();
                        let mut child_command = Command::new("git").args(["commit", "-m", message]).spawn().unwrap();
                        child_command.wait();
                    } else {
                        let mut child_command = Command::new("git").args(args).spawn().unwrap();
                        let _ = child_command.wait();
                    }
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.take().map_or(Stdio::inherit(), |child: std::process::Child| child.stdout.unwrap().into());
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    
                    match output {
                        Ok(child) => {
                            previous_command = Some(child);
                        }
                        Err(err) => {
                            previous_command = None;
                            eprintln!("{}: {}", command, err);
                        }
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            let _ = final_command.wait();
        }
    }
}

