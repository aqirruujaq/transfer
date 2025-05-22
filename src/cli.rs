mod server;

use std::{
    io::{self, Write},
    str::SplitWhitespace,
};

use server::Serve;

/// The main entry point for the CLI application.
#[derive(Default)]
pub struct Cli {
    serve: Serve,
}

impl Cli {
    /// runs the CLI application.
    pub fn run(mut self) {
        let mut input = String::new();
        // Flag indicating whether the program should keep running.
        // Set to false to exit the loop.
        let mut running = true;
        while running {
            print!("transfer>");
            let _ = io::stdout().flush();
            input.clear();

            running = match io::stdin().read_line(&mut input) {
                Ok(_) => self.handle_input(&input),
                Err(e) => {
                    println!("read_line err: {}", e);
                    true
                }
            };
        }
    }

    /// Start the transfer server.
    fn start(&mut self, mut exp: SplitWhitespace) -> bool {
        if let Some(port) = exp.next() {
            if exp.next().is_none() {
                let port = port.parse::<u16>();
                match port {
                    Ok(port) => self.serve.start(port),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!(
                    "Error: `start` expects exactly one argument (`port`), but received multiple."
                )
            }
        } else {
            println!("Error: `start` expects exactly one argument (`port`), but received zero.")
        }
        true
    }

    /// Handle user input and execute the corresponding commands.
    /// The bool indicates whether to continue running the program.
    fn handle_input(&mut self, input: &str) -> bool {
        let mut exp = input.split_whitespace();
        if let Some(cmd) = exp.next() {
            // The first word is the command.
            // The rest are arguments.
            // Matches command names to functions whit the same name.
            // Each function takes a SplitWhitespace iterator and returns a bool.
            match cmd {
                "start" => self.start(exp),
                "quit" | "exit" => exit(exp),
                "reboot" => todo!(),
                "help" => help(exp),
                _ => {
                    println!(
                        "no find command: `{}`, \n\nhelp: view all commands whit `help`",
                        cmd
                    );
                    true
                }
            }
        } else {
            true
        }
    }
}

fn exit(mut exp: SplitWhitespace) -> bool {
    if exp.next().is_none() {
        false
    } else {
        println!("no additional parameters are required for exit");
        true
    }
}

// Display help information for the user.
fn help(_: SplitWhitespace) -> bool {
    println!(
        r#"commands:
        --start: start transfer serve
        --exit | quit: exit the program
        --help: Display helpful information for user"#
    );
    true
}
