use std::net::TcpStream;
use std::io::{self, Read, Write};

use acciones::accion::Accion;
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

fn generar_accion() -> Accion {
    let mut accion = String::new();
    println!("Elige una accion (m: moverse, a: atacar, t: abrir la tienda, s:saltar): ");
    io::stdin()
    .read_line(&mut accion)
    .expect("Error al leer la entrada");
/* 
    match accion.trim() {
        "m" => return self.moverse(),
        "a" => return self.atacar(),
        "t" => return self.abrir_tienda(),
        "s" => return Accion::Saltar,
        _ => return Accion::Saltar,
    }
    */
    return Accion::Saltar
}

fn main() {
        // Conectarse al servidor
        let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

        let mut estado_mapa_bytes = vec![]; // Vector para almacenar los bytes recibidos
        let mut buffer = [0; 40000]; // Buffer temporal para leer datos

        // Leer hasta que no haya más datos disponibles
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break, // La conexión se ha cerrado
                Ok(n) => estado_mapa_bytes.extend_from_slice(&buffer[..n]),
                Err(e) => {
                    eprintln!("Failed to read from server: {}", e);
                    return;
                }
            }
            let mapa_actual: Mapa = from_bytes(&estado_mapa_bytes);
            mapa_actual.imprimir();
            imprimir_acciones();
    
            // Aquí puedes realizar cualquier procesamiento necesario con el estado del mapa recibido
    
            // Enviar una acción al servidor
            let action = generar_accion();
            //stream.write(action.as_bytes()).unwrap();
        }
    
}
