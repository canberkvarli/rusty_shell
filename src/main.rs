use std::env;
use std::io::{Write, stdin, stdout};
use std::path::Path;
use std::process::Command;
use std::process::{Child, Stdio};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        let mut commands: std::iter::Peekable<std::str::Split<'_, &str>> =
            input.trim().split(" | ").peekable();
        let mut previous_command: Option<Child> = None;

        while let Some(command) = commands.next() {
            let mut parts: std::str::SplitWhitespace<'_> = command.trim().split_whitespace();
            let command: &str = parts.next().unwrap();
            let args: std::str::SplitWhitespace<'_> = parts;

            match command {
                "cd" => {
                    let new_dir: &str = args.peekable().peek().map_or("/", |x: &&str| *x);
                    let root: &Path = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin: Stdio = previous_command
                        .map_or(Stdio::inherit(), |output: Child| {
                            Stdio::from(output.stdout.unwrap())
                        });

                    let stdout: Stdio = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output: Result<Child, std::io::Error> = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            let _ = final_command.wait();
        }
    }
}
