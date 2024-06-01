use std::net::TcpStream;
use std::io::{Read, Write};

use barcos::flota::Flota;
use juego::mapa::Mapa;
use ndarray::Array2;

pub fn from_bytes(bytes: &[u8]) -> Mapa {
    // Lee el tablero de los bytes
    let mut tablero = Array2::from_elem((10, 10), '.');
    for (i, byte) in bytes.iter().enumerate().take(10 * 10) {
        tablero[[i / 10, i % 10]] = *byte as char;
    }

    // Lee las flotas de los bytes
    let mut flotas = Vec::new();
    let mut offset = 10 * 10; // Desplazamiento inicial después del tablero
    while offset < bytes.len() {
        let (flota, bytes_read) = Flota::from_bytes(&bytes[offset..]);
        flotas.push(flota);
        offset += bytes_read;
    }

    Mapa { tablero, flotas }
}

fn imprimir_acciones() {
    println!("Realice una accion: ");
    println!("Puede moverse: (m)");
    println!("Puede atacar: (a)");
    println!("Puede abrir la tienda: (t)");
    println!("Puede saltar turno: (s)");
    
}

fn main() {
    loop {
        // Conectarse al servidor
        let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

        // Leer la respuesta del servidor (estado del mapa)
        let mut estado_mapa_bytes = [0; 1024]; // Buffer para almacenar los bytes recibidos
        let bytes = stream.read(&mut estado_mapa_bytes).expect("Failed to read map state from server");
        println!("Received map state from server");

        let mapa_actual: Mapa = from_bytes(&estado_mapa_bytes[..bytes]);
        mapa_actual.imprimir();
        imprimir_acciones();

        // Aquí puedes realizar cualquier procesamiento necesario con el estado del mapa recibido

        // Enviar una acción al servidor
        let action = "ActionFromClient";
        stream.write(action.as_bytes()).unwrap();
        println!("Action sent to server: {}", action);

        // Pausa para evitar solicitudes excesivas al servidor
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
