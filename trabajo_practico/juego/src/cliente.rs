use libreria::{
    constantes::{ATAQ, MOV},
    custom_error::CustomError,
};
use serde_json;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::mensaje::{Instruccion, Mensaje};
/// Struct que representa un cliente
pub struct Cliente {
    shared_stream: Arc<Mutex<TcpStream>>,
    _id: usize,
    nombre: String,
    _puntos: u32,
}

impl Cliente {
    /// Función que crea un nuevo cliente
    /// 
    /// # Args
    /// 
    /// `addr` - Dirección del servidor
    /// 
    /// `_id` - Identificador del cliente
    /// 
    /// `nombre` - Nombre del cliente
    /// 
    /// # Returns
    /// 
    /// `Result<Self, CustomError>` - Resultado de la creación del cliente
    pub fn new(addr: String, _id: usize, nombre: String) -> Result<Self, CustomError> {
        let stream = TcpStream::connect(addr).map_err(|_| CustomError::ErrorCreatingSocket)?;
        let shared_stream: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(stream));
        Ok(Cliente {
            shared_stream,
            _id,
            nombre,
            _puntos: 0,
        })
    }
    /// Función que ejecuta el cliente
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la ejecución del cliente
    /// 
    /// # Errors
    /// 
    /// Retorna un error si no se puede recibir el mensaje del servidor
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
                                Mensaje::Registro => {
                                    println!("Ingrese su nombre de usuario: ");
                                    let mut respuesta = String::new();
                                    io::stdin()
                                        .read_line(&mut respuesta)
                                        .expect("Error al leer la respuesta.");
                                    self.enviar_respuesta(respuesta.trim())?;
                                }
                                Mensaje::PreguntaComienzo => {
                                    println!("¿Ya hay jugadores suficientes.Deseas comenzar el juego? (si/no)");
                                    let mut respuesta = String::new();
                                    io::stdin()
                                        .read_line(&mut respuesta)
                                        .expect("Error al leer la respuesta.");
                                    self.enviar_respuesta(respuesta.trim())?;
                                }
                                Mensaje::RealiceAccion => {
                                    Self::imprimir_acciones();
                                }
                                Mensaje::Esperando => {
                                    println!("Esperando mas jugadores para comenzar el juego...");
                                }
                                Mensaje::Puntos(puntos) => {
                                    println!("Puntos: {}", puntos);
                                }
                                Mensaje::Tablero(tablero, barcos, monedas) => {
                                    for row in tablero {
                                        for cell in row {
                                            print!("{}", cell);
                                        }
                                        println!();
                                    }
                                    let accion = Self::pedir_instrucciones(barcos, monedas);
                                    let mensaje_serializado =
                                        serde_json::to_string(&Mensaje::Accion(accion)).unwrap();
                                    self.enviar_respuesta(mensaje_serializado.as_str())?;
                                }
                                Mensaje::RepetirAccion(mensaje, barcos, monedas) => {
                                    println!("{}", mensaje);
                                    let accion = Self::pedir_instrucciones(barcos, monedas);
                                    let mensaje_serializado =
                                        serde_json::to_string(&Mensaje::Accion(accion)).unwrap();
                                    self.enviar_respuesta(mensaje_serializado.as_str())?;
                                }
                                Mensaje::EventoSorpresa => {
                                    println!("Un cargamento con recursos aparecio de repente! se el primero en reclamarlo ingresando: primero");
                                    let mut respuesta = String::new();
                                    io::stdin()
                                        .read_line(&mut respuesta)
                                        .expect("Error al leer la respuesta.");
                                    self.enviar_respuesta(respuesta.trim())?;
                                }
                                Mensaje::MensajeInfoAaque(puntos,monedas ) =>{
                                    println!("Has golpeado a un barco enemigo, has ganado {} puntos y {} monedas", puntos, monedas);
                                }
                                Mensaje::EventoSorpresaResultado(resultado) => {
                                    if resultado {
                                        println!("Felicidades, fuiste el primero en reclamar el premio, ahora es tuyo");
                                    } else {
                                        println!("Una lastima, alguien se te adelanto, perdiste el premio");
                                    }
                                }
                                Mensaje::Ranking(ranking) => {
                                    Self::mostrar_ranking(ranking)?;
                                }
                                Mensaje::FinPartida(nombre, puntos) => {
                                    println!("Fin de la partida. El jugador {} ha ganado con {} puntos", nombre, puntos);
                                    break;
                                }
                                _ => {
                                    Err(CustomError::ErrorRecibiendoMensaje)?;
                                }
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
    /// Función que envía una respuesta al servidor
    /// 
    /// # Args
    /// 
    /// `respuesta` - Respuesta a enviar al servidor
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado del envío de la respuesta
    /// 
    /// # Errors
    /// 
    /// Retorna un error si no se puede enviar la respuesta
    pub fn enviar_respuesta(&mut self, respuesta: &str) -> Result<(), CustomError> {
        let mut stream = self.shared_stream.lock().unwrap();
        stream
            .write_all(respuesta.as_bytes())
            .map_err(|_| CustomError::ErrorEnviarMensaje)?;
        stream
            .flush()
            .map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }
    /// Función que muestra el ranking de los jugadores
    /// 
    /// # Args
    /// 
    /// `ranking` - Ranking de los jugadores
    /// 
    /// # Returns
    /// 
    /// `()` - No retorna nada
    fn mostrar_ranking(ranking: Vec<(String, usize)>) -> Result<(), CustomError>{
        if ranking.is_empty() {
            return Err(CustomError::ErrorRankingVacio);
        }
        println!("Ranking:");
        for (index, (nombre, puntos)) in ranking.iter().enumerate() {
            println!("{:<5} {:<15} {:<10}", index + 1, nombre, puntos);
        }
        Ok(())
    }

    /// Funcion que permite al jugador abrir la tienda y comprar barcos
    /// # Args
    /// `coordenadas_ataque` - Coordenadas del ataque realizado por el jugador
    /// 
    /// `mapa` - Mapa en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// `usize` - Puntos ganados por el jugador
    fn abrir_tienda(monedas: usize) -> Instruccion {
        println!("Opciones para comprar: ");
        println!("(a) Acorazado de 3 casilleros: $300");
        println!("(b) Buqe de 2 casilleros: $200");
        println!("(c) Fragata de 1 casillero: $100");
        println!("Usted cuenta con ${}", monedas);
        let mut compra = String::new();
        io::stdin()
            .read_line(&mut compra)
            .expect("Error al leer la respuesta.");
        let mut exitosa = true;
        let mut tipo_barco = 0;
        match compra.trim() {
            "a" => {
                if monedas < 300 {exitosa = false}
                tipo_barco = 3;
            },
            "b" => {
                if monedas < 200 {exitosa = false}
                tipo_barco = 2;
            },
            "c" => {
                if monedas < 100 {exitosa = false}
                tipo_barco = 1;
            },
            _ => {}
        }
        if !exitosa {
            println!("No cuenta con el dinero suficiente para comprar ese barco");
            Instruccion::Saltar
        } else {
            Instruccion::Compra(tipo_barco)
        }
    }

    pub fn recibir_mensaje(&mut self) -> Result<String, CustomError> {
        let mut buffer = [0; 2048];
        let mut stream = self.shared_stream.lock().unwrap();
        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|_| CustomError::ErrorRecibiendoInstruccion)?;
        let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(message)
    }
    pub fn cambiar_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
    pub fn obtener_nombre(&self) -> String {
        self.nombre.clone()
    }
    fn imprimir_acciones() {
        println!("Realice una accion: ");
        println!("Puede moverse: (m)");
        println!("Puede atacar: (a)");
        println!("Puede abrir la tienda: (t)");
        println!("Puede saltar turno: (s)");
        println!("Puede ver el ranking: (r)")
    }

    fn pedir_instrucciones(barcos: Vec<(usize, Vec<(i32, i32)>)>, monedas: usize) -> Instruccion {
        let mut accion = String::new();
        io::stdin()
            .read_line(&mut accion)
            .expect("Error al leer la entrada");

        match accion.trim() {
            "m" => Self::moverse(barcos).unwrap(),
            "a" => Self::atacar(barcos).unwrap(),
            "t" => Self::abrir_tienda(monedas),
            "s" => Instruccion::Saltar,
            "r" => Instruccion::Ranking,
            _ => {
                println!("Error en la accion. Por favor, elige una accion valida (m, a, t, s).");
                Self::pedir_instrucciones(barcos, monedas)
            }
        }
    }

    fn moverse(barcos: Vec<(usize, Vec<(i32, i32)>)>) -> Result<Instruccion,CustomError> {
        let (id, posicion) = Self::obtener_barco(barcos, MOV).unwrap();
        Ok(Instruccion::Movimiento(id, posicion))
    }

    fn atacar(barcos: Vec<(usize, Vec<(i32, i32)>)>) ->Result<Instruccion, CustomError>{
        let (id, posicion) = Self::obtener_barco(barcos, ATAQ).unwrap();
        
        Ok(Instruccion::Ataque(id, posicion))
    }

    fn obtener_barco(barcos: Vec<(usize, Vec<(i32, i32)>)>, accion: &str) -> Result<(usize, (i32, i32)),CustomError>{
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
                println!(
                    "Numero de barco invalido. Por favor, ingrese un numero dentro del rango."
                );
                return Self::obtener_barco(barcos, accion);
            }
        };

        if barco_seleccionado >= barcos.len() {
            println!("Numero de barco invalido. Por favor, elige un numero dentro del rango.");
            return Self::obtener_barco(barcos, accion);
        }

        let cordenadas = Self::pedir_coordenadas().unwrap();
        Ok((barco_seleccionado, cordenadas))
    }
    /// Función que pide las coordenadas al usuario
    /// 
    /// # Returns
    /// 
    /// `(i32, i32)` - Coordenadas ingresadas por el usuario
    /// 
    /// # Errors
    /// 
    /// Retorna un error si las coordenadas ingresadas no son válidas
    fn pedir_coordenadas() -> Result<(i32, i32), CustomError>{
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
                        return Ok((x, y));
                    }
                }
            }

            return Err(CustomError::ErrorCoordenadasIncorrectas);
        }
    }
}
