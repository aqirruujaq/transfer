use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(multicall = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

// TODO: Add command `clear` `pause` `drop`
#[derive(Subcommand)]
pub enum Commands {
    /// exit the transfer
    Exit,
    /// Start transfer server
    Start {
        /// Port to be monitored
        #[arg(default_value = "8080")]
        port: u16,
    },
    /// Check the server status
    Status,
    /// Display listening port
    Port,
}
