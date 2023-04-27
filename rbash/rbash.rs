use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::env;

fn main() {
    loop {
        print!("rbash> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args = parts.collect::<Vec<&str>>();
        match command {
            "ls" => {
                let output = Command::new("ls")
                    .args(args)
                    .output()
                    .expect("failed to execute command");
                io::stdout().write_all(&output.stdout).unwrap();
            },
            "echo" => {
                println!("{}", args.join(" "));
            },
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            },
            "tar" => {
                let output = Command::new("tar")
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("failed to execute command");
            },
            "help" => {
                println!("Available commands:");
                println!("ls - list directory contents");
                println!("echo - print input to output");
                println!("pwd - print current working directory");
                println!("tar - archive files into a tarball");
                println!("help - display this help message");
            },
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
