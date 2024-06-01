use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    // Conectarse al servidor
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    // Enviar una solicitud al servidor
    let request = "Hello from client!";
    stream.write(request.as_bytes()).unwrap();
    println!("Request sent: {}", request);

    // Leer la respuesta del servidor
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("Failed to read response from server");
    println!("Response from server: {}", response);
}
