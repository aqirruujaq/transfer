mod file_tree;
mod https_parse;

use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use https_parse::response;

/// Thread responsible for running the transfer server.
/// This serve based
#[derive(Default)]
pub struct Serve {
    thread: Option<thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
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
        let state = self.running.load(Ordering::SeqCst);
        match state {
            false => "The serve is not running",
            true => "The serve is running",
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
        let running = Arc::clone(&self.running);
        self.port = Some(port);
        self.thread = Some(thread::spawn(move || start_server(port, running)));
    }
}

fn start_server(port: u16, running: Arc<AtomicBool>) {
    let tcplister = if let Ok(tl) = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))) {
        tl
    } else {
        return;
    };

    running.store(true, Ordering::SeqCst);
    for stream in tcplister.incoming() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        match stream {
            Ok(mut stream) => {
                stream
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .expect("No error will occur");
                handle_connection(&mut stream);
            }
            Err(_) => todo!(),
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    response(stream);
}

impl Drop for Serve {
    fn drop(&mut self) {
        if let Some(serve) = self.thread.take() {
            self.running.store(false, Ordering::SeqCst);
            TcpStream::connect(format!("127.0.0.1:{}", self.port.unwrap())).unwrap();
            serve.join().unwrap();
        }
    }
}
