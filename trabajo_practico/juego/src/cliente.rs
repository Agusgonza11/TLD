use std::{
    io::{ self, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use libreria::custom_error::CustomError;


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
                Ok(mensaje) => {
                    if mensaje.trim() == "¿Desean comenzar el juego? (si/no)" {
                        println!("¿Ya hay jugadores suficientes.Deseas comenzar el juego? (si/no)");
                        let mut respuesta = String::new();
                        io::stdin().read_line(&mut respuesta).expect("Error al leer la respuesta.");
                        self.enviar_respuesta(respuesta.trim())?;
                    }
                    if mensaje.starts_with("Realice una accion:") || mensaje.starts_with("Esperando"){
                        println!("{}", mensaje);
                    }
                    if mensaje.starts_with("Puntos:") {
                        println!("{}", mensaje);
                        
                        let mut respuesta = String::new();
                        io::stdin().read_line(&mut respuesta).expect("Error al leer la respuesta.");
                        self.enviar_respuesta(respuesta.trim())?;
                    }
                    if mensaje.starts_with("TABLERO:") {
                        let contenido_tablero = &mensaje["TABLERO:".len()..]; 
                        
                        match serde_json::from_str::<Vec<Vec<char>>>(contenido_tablero) {
                            Ok(tablero) => {
                                for row in tablero {
                                    for cell in row {
                                        print!("{}", cell);
                                    }
                                    println!();
                                }
                            }
                            Err(err) => {
                                println!("Error al deserializar el tablero: {}", err);
                            }
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
        let mut buffer = [0; 512];
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

}
