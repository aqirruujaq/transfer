mod cli;

pub fn run() {
    let cli = cli::Cli::default();
    cli.run();
}