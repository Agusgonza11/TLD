use std::{
    io::{ self, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};
use serde_json;
use libreria::{constantes::{ATAQ, MOV}, custom_error::CustomError};

use crate::mensaje::{Instruccion, Mensaje};


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
                                    let accion = Self::pedir_instrucciones(barcos);
                                    let mensaje_serializado = serde_json::to_string(&Mensaje::Accion(accion)).unwrap();
                                    self.enviar_respuesta(mensaje_serializado.as_str())?;

                                },
                                Mensaje::RepetirAccion(mensaje, barcos) => {
                                    println!("{}", mensaje);
                                    let accion = Self::pedir_instrucciones(barcos);
                                    let mensaje_serializado = serde_json::to_string(&Mensaje::Accion(accion)).unwrap();
                                    self.enviar_respuesta(mensaje_serializado.as_str())?;
                                }
                                _ => {}
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
        //stream.flush().map_err(|_| CustomError::ErrorEnviarMensaje)?;
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

    fn pedir_instrucciones(barcos: Vec<(usize, Vec<(i32, i32)>)>) -> Instruccion {
        let mut accion = String::new();
        io::stdin()
        .read_line(&mut accion)
        .expect("Error al leer la entrada");
        
        match accion.trim() {
            "m" => Self::moverse(barcos),
            "a" => Self::atacar(barcos),
            "t" => Instruccion::Tienda,
            "s" => Instruccion::Saltar,
            _ => {
                println!("Error en la accion. Por favor, elige una accion valida (m, a, t, s).");
                Instruccion::Saltar
            },   
        }
        
    }

    fn moverse(barcos: Vec<(usize, Vec<(i32, i32)>)>) -> Instruccion {
        let (id, posicion) = Self::obtener_barco(barcos, MOV);
        Instruccion::Movimiento(id, posicion)
    }

    fn atacar(barcos: Vec<(usize, Vec<(i32, i32)>)>) -> Instruccion {
        let (id, posicion) = Self::obtener_barco(barcos, ATAQ);

        Instruccion::Ataque(id, posicion)
    }

    fn obtener_barco(barcos: Vec<(usize, Vec<(i32, i32)>)>, accion: &str) -> (usize, (i32, i32)) {
        println!("Elige un barco para {}:", accion);
        for (i, (id, posicion)) in barcos.iter().enumerate() {
            println!("{}: ID: {}, Posicion: {:?}", i, id, posicion);
        }

        let mut barco_seleccionado = String::new();
        io::stdout().flush().expect("Error");
        io::stdin()
            .read_line(&mut barco_seleccionado)
            .expect("Error");
        let barco_seleccionado: usize = match barco_seleccionado.trim().parse() {
            Ok(numero) => numero,
            Err(_) => {
                println!("Numero de barco invalido. Por favor, ingrese un numero dentro del rango.");
                return Self::obtener_barco(barcos, accion);
            }
        };

        if barco_seleccionado >= barcos.len() {
            println!("Numero de barco invalido. Por favor, elige un numero dentro del rango.");
            return Self::obtener_barco(barcos, accion);
        }

        let cordenadas = Self::pedir_coordenadas();

        (barco_seleccionado, cordenadas)
    }


    fn pedir_coordenadas() -> (i32, i32) {
        loop {
            println!("Ingresa las coordenadas en formato 'x,y': ");

            let mut coordenadas = String::new();
            io::stdin()
                .read_line(&mut coordenadas)
                .expect("Error al leer la entrada");

            let mut iter = coordenadas.trim().split(',');
            if let (Some(x_str), Some(y_str)) = (iter.next(), iter.next()) {
                if let Ok(x) = x_str.trim().parse::<i32>() {
                    if let Ok(y) = y_str.trim().parse::<i32>() {
                        return (x, y);
                    }
                }
            }

            println!("Formato de coordenadas incorrecto. Intentalo de nuevo.");
        }
    }

}
    

