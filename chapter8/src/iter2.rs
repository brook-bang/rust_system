use std::{
    io::{Write, stdin, stdout},
    process::Command,
    thread::spawn,
};

fn main() {
    loop {
        print!("$");
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Unable to read user input");
        let command_to_execute = user_input.trim();
        let command_args: Vec<&str> = command_to_execute.split_ascii_whitespace().collect();
        let mut child = Command::new(command_args[0])
            .args(&command_args[1..])
            .spawn()
            .expect("Unable to execute command");
        child.wait().unwrap();
    }
}
