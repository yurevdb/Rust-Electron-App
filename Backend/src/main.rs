use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::prelude::*;

/// Main entry point to the backend server
fn main() {
    // Check if Unix Socket file exists
    // If so, delete it
    if std::path::Path::new("/tmp/rusttron.test").exists() {
        std::fs::remove_file("/tmp/rusttron.test").unwrap();
    }

    // Create a listener for the Unix Socket server
    let listener = UnixListener::bind("/tmp/rusttron.test").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connected to a client.");
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }
}

/// Handles every unix socket client
fn handle_client(mut stream: UnixStream) {
    stream.write_all(b"Hello World from server").unwrap();

    loop {
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        if response.len() > 0 {
            println!("Received data: {}", response);
        }
    }
}
