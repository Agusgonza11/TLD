use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use acciones::accion::Accion;
use juego::mapa::Mapa;
use juego::juego::Juego;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

struct Mensaje {
    mapa: Juego,
}

fn handle_client(mut stream: TcpStream, juego: Arc<Mutex<Juego>>) {
    // Lógica para manejar la comunicación con el cliente
    let mut buffer = [0; 1024];
    loop {
        // Envía un mensaje al cliente indicándole que es su turno
        let copia_juego = juego.lock().unwrap();

        stream.write(&copia_juego.mapa.clone().to_bytes()).unwrap();

        // Lee la respuesta del cliente, que será de tipo Accion
        let mut response = Vec::new();
        stream.read_to_end(&mut response).expect("Failed to read from client");
        let response_str = String::from_utf8_lossy(&response);
        let accion: Accion = serde_json::from_str(&response_str).unwrap();

        // Ejemplo de cómo acceder y modificar el mapa compartido
        {
            let mut juego = juego.lock().unwrap(); // Bloquea el mapa para acceder a él de manera segura
            juego.modificar(accion); // Modifica el mapa según la respuesta del cliente
        }

    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on port 8080...");

    // Crea un objeto Mapa compartido
    let juego = Arc::new(Mutex::new(Juego::new()));
    let jugadores = Arc::new(AtomicUsize::new(0));

    // Acepta conexiones entrantes y las maneja en subprocesos
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Clona el juego compartido para pasarlo al hilo
                let juego_clone = Arc::clone(&juego);

                // Clona el juego compartido nuevamente para pasarlo al cierre
                let juego_clone_thread = Arc::clone(&juego_clone);

                // Incrementa el contador de jugadores y obtén el nuevo ID
                let jugador_id = jugadores.fetch_add(1, Ordering::SeqCst);

                // Almacena el identificador del hilo en el vector
                let handle = thread::spawn(move || {
                    let mut juego = juego_clone_thread.lock().unwrap();
                    juego.agregar_jugador(jugador_id); // Agrega un jugador al juego con el ID único
                    handle_client(stream, juego_clone);
                });
                handle.join().unwrap(); // Espera a que el hilo maneje el cliente antes de pasar al siguiente cliente
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
