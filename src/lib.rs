use clap::Parser;
use cli::{Cli, Commands};
use rustyline::{DefaultEditor, error::ReadlineError};
use server::Serve;

mod cli;
mod server;

#[derive(Default)]
pub struct Transfer {
    serve: Serve,
}

impl Transfer {
    pub fn respond(&mut self, line: &str) -> Result<bool, String> {
        let cli = Cli::try_parse_from(line.split_whitespace()).map_err(|e| e.to_string())?;
        match cli.cmd {
            Commands::Exit => {
                println!("Exiting server");
                return Ok(true);
            }
            Commands::Start { port } => {
                self.serve.start(port);
            }
            Commands::Status => {
                println!("{}", self.serve.state());
            }
            Commands::Port => {
                println!(
                    "{}",
                    match self.serve.port() {
                        Some(port) => port.to_string(),
                        None => "No listening port".to_string(),
                    }
                );
            }
        }
        Ok(false)
    }
}

pub fn run() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut transfer = Transfer::default();
    // TODO: Add a configuration file and detect whether to automatically start the service from the configuration file
    transfer.serve.start(8080);
    loop {
        let line = match rl.readline("transfer> ") {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                } else {
                    line
                }
            }
            Err(readlineerror) => match readlineerror {
                ReadlineError::Interrupted => {
                    println!("CTRL-C");
                    break;
                }
                ReadlineError::Eof => {
                    println!("CTRL-D");
                    break;
                }
                _ => {
                    println!("readline error: {}", readlineerror);
                    continue;
                }
            },
        };

        match transfer.respond(&line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
