use std::{
    io::{ self, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};
use serde_json;

use libreria::custom_error::CustomError;

use crate::mensaje::Mensaje;


pub struct Cliente {
    shared_stream: Arc<Mutex<TcpStream>>,
    _id: usize,
    nombre: String,
    _puntos: u32,
}

impl Cliente {
    pub fn new(addr: String,_id: usize, nombre: String) -> Result<Self, CustomError> {
        let stream = TcpStream::connect(addr).map_err(|_| CustomError::ErrorCreatingSocket)?;
        let shared_stream: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(stream));
        Ok(Cliente {
            shared_stream,
            _id,
            nombre,
            _puntos: 0,
            
        })
    }

    pub fn run(&mut self) -> Result<(), CustomError> {
        loop {
            match self.recibir_mensaje() {
                Ok(mensaje_serializado) => {
                    if mensaje_serializado.is_empty() {
                        println!("Mensaje vacío recibido");
                        continue;
                    }
                    match serde_json::from_str::<Mensaje>(&mensaje_serializado) {
                        Ok(mensaje) => {
                            match mensaje {
                                Mensaje::PreguntaComienzo => {
                                    println!("¿Ya hay jugadores suficientes.Deseas comenzar el juego? (si/no)");
                                    let mut respuesta = String::new();
                                    io::stdin().read_line(&mut respuesta).expect("Error al leer la respuesta.");
                                    self.enviar_respuesta(respuesta.trim())?;
                                },
                                Mensaje::RealiceAccion => {
                                    Self::imprimir_acciones();
                                },
                                Mensaje::Esperando => {
                                    println!("Esperando mas jugadores para comenzar el juego...");
                                },
                                Mensaje::Puntos(puntos) => {
                                    println!("Puntos: {}", puntos);
                                },
                                Mensaje::Tablero(tablero, barcos) => {
                                    for row in tablero {
                                        for cell in row {
                                            print!("{}", cell);
                                        }
                                        println!();
                                    }
                                    // Aquí se podría enviar la acción del jugador
                                    //let accion = "accion del jugador";  // Cambia esto por la acción real del jugador
                                    //self.enviar_tablero(tablero, vec![accion.chars().next().unwrap()])?;  // Ejemplo de envío de un tablero
                                },
                            }
                        }
                        Err(err) => {
                            println!("Error al deserializar el mensaje: {}", err);
                        }
                    }
                }
                Err(e) => {
                    println!("Error al recibir mensaje del servidor: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }
    

    pub fn enviar_respuesta(&mut self, respuesta: &str) -> Result<(), CustomError> {
        let mut stream = self.shared_stream.lock().unwrap();
        stream.write_all(respuesta.as_bytes()).map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }


    pub fn recibir_mensaje(&mut self) -> Result<String, CustomError> {
        let mut buffer = [0; 2048];
        let mut stream = self.shared_stream.lock().unwrap(); 
        let bytes_read = stream.read(&mut buffer).map_err(|_| CustomError::ErrorRecibiendoInstruccion)?;
        let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(message)
    }
    pub fn cambiar_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
    pub fn obtener_nombre(&self) -> String {
        self.nombre.clone()
    }
    fn _enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(),CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }

    fn imprimir_acciones() {
        println!("Realice una accion: ");
        println!("Puede moverse: (m)");
        println!("Puede atacar: (a)");
        println!("Puede abrir la tienda: (t)");
        println!("Puede saltar turno: (s)");
        
    }
    

}
