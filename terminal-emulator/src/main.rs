use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
                
        //args are just going to be the words after the first word which is the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        let mut child_command = Command::new(command).args(args).spawn().unwrap();
        child_command.wait();
    }


}
