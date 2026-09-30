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

pub fn connect_to_server() -> TcpStream{
    let mut stream = TcpStream::connect("127.0.0.1:6767").expect("Failed to connect to server");
    println!("Connected to server!");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from server");
    let response = String::from_utf8_lossy(&buffer);
    println!("Received: {}", response);
    stream
}

pub fn start_server() -> TcpStream {
    let listener = TcpListener::bind("127.0.0.1:6767").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:6767");
    let (mut stream, _) = listener.accept().expect("Failed to accept connection");
    stream.write_all(b"W\n").expect("Failed to send color");
    stream
}