use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(multicall = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// exit the transfer
    Exit,
    /// Start transfer server
    Start,
    /// Check the server status
    Status,
    /// Display listening port
    Port,
}
