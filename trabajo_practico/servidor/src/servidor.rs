use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // Lógica para manejar la comunicación con el cliente
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            // El cliente cerró la conexión
            break;
        }
        // Procesar la solicitud del cliente
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request: {}", request);
        
        // Aquí podrías procesar la solicitud y enviar una respuesta
        let response = "Hello from server!";
        stream.write(response.as_bytes()).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on port 8080...");

    // Vector para almacenar los identificadores de los hilos
    let mut handles = vec![];

    // Acepta conexiones entrantes y las maneja en subprocesos
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Almacena el identificador del hilo en el vector
                let handle = thread::spawn(move || {
                    handle_client(stream);
                });
                handles.push(handle);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    // Espera a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }
}
