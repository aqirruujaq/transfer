use std::{
    io::{BufRead, BufReader},
    net::SocketAddr,
    thread,
};

/// Thread responsible for running the transfer server.
/// This serve based
#[derive(Default)]
pub struct Serve {
    thread: Option<thread::JoinHandle<()>>,
}

impl Serve {
    /// Detemines if the server is active and operational.
    /// If the server is running, it returns true.
    pub fn is_running(&self) -> bool {
        self.thread.is_some()
    }

    pub fn start(&mut self, port: u16) {
        // Determine whether to start
        if self.is_running() {
            println!("serve is already running");
            return;
        }
        if port < 1024 || port > 49151 {
            println!("port {port} is not in range");
            return;
        }

        // Start the server in a new thread.
        self.thread = Some(thread::spawn(move || {
            let tcplister = if let Ok(tl) =
                std::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
            {
                tl
            } else {
                println!("Cannot bind to 0.0.0.0:{}", port);
                return;
            };
            println!("Connection established successfully");
            for stream in tcplister.incoming() {
                match stream {
                    Ok(stream) => {
                        let buf = BufReader::new(&stream);
                        for line in buf.lines() {
                            let line = line.unwrap();
                            println!("{}", line);
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }));
    }
}
