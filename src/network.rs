use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from stream");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "hello!".as_bytes();
    stream.write(response).expect("Failed to write to stream");
}

pub fn connect_to_server() {
    let mut stream = TcpStream::connect("127.0.0.1:6767").expect("Failed to connect to server");
    println!("Connected to server!");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from server");
    let response = String::from_utf8_lossy(&buffer);
    println!("Received: {}", response);
}

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:6767").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:6767");
    for stream in listener.incoming() {
        handle_client(stream.expect("Failed to accept connection"));
    }
}