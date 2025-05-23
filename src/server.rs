use std::{
    io::{BufRead, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

/// Thread responsible for running the transfer server.
/// This serve based
#[derive(Default)]
pub struct Serve {
    thread: Option<thread::JoinHandle<()>>,
    state: Arc<Mutex<State>>,
    port: Option<u16>,
}

impl Serve {
    /// Detemines if the server is active and operational.
    /// If the server is running, it returns true.
    pub fn is_running(&self) -> bool {
        self.thread.is_some()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn state(&self) -> &str {
        let state = *self.state.lock().unwrap();
        match state {
            State::NotRunning => "The serve is not running",
            State::Running => "The serve is running",
        }
    }

    pub fn start(&mut self, port: u16) {
        // Determine whether to start
        if self.is_running() {
            println!("serve is already running");
            return;
        }
        if !(1024..=49151).contains(&port) {
            println!("port {port} is not in range");
            return;
        }

        // Start the server in a new thread.
        let state = Arc::clone(&self.state);
        self.port = Some(port);
        self.thread = Some(thread::spawn(move || start_server(port, state)));
    }
}

fn start_server(port: u16, state: Arc<Mutex<State>>) {
    let tcplister = if let Ok(tl) = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))) {
        tl
    } else {
        return;
    };

    *state.lock().unwrap() = State::Running;
    for stream in tcplister.incoming() {
        if *state.lock().unwrap() == State::NotRunning {
            break;
        }

        match stream {
            Ok(stream) => {
                let buf = BufReader::new(&stream);
                for line in buf.lines() {
                    let line = line.unwrap();
                    println!("{}", line);
                }
            }
            Err(_) => todo!(),
        }
    }
}

impl Drop for Serve {
    fn drop(&mut self) {
        if let Some(serve) = self.thread.take() {
            *self.state.lock().unwrap() = State::NotRunning;
            TcpStream::connect(format!("127.0.0.1:{}", self.port.unwrap())).unwrap();
            serve.join().unwrap();
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum State {
    // The server is running.
    Running,
    // The server is not running
    #[default]
    NotRunning,
}
