use std::{io::Write, net::TcpStream, sync::MutexGuard};

use barcos::{barco::Barco, estado_barco::EstadoBarco};
use libreria::custom_error::CustomError;
use crate::juego::CustomError::AccionInvalida;
use crate::{jugador::Jugador, mapa::Mapa, mensaje::{Instruccion, Mensaje}, server::Server};

#[derive(Clone)]
pub struct Juego {
    pub mapa: Mapa,
    pub jugadores: Vec<Jugador>,
    pub turno: usize,
}

impl Juego {
    /// Función que crea un nuevo juego
    /// 
    /// # Args
    /// 
    /// `numero_jugadores` - Número de jugadores que participarán en el juego
    /// 
    /// # Returns
    /// 
    /// `Juego` - Juego creado
    /// 
    pub fn new(numero_jugadores: usize) -> Juego {
        let mut mapa = Mapa::new();
        let mut jugadores = Vec::new();
        for _ in 0..numero_jugadores {
            jugadores.push(Jugador::new(jugadores.len(), &mut mapa));
        }
        let turno = 0;
        Juego {
            mapa,
            jugadores,
            turno,
        }
    }
    /// Función que inicia el juego
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la ejecución
    /// 
    /// # Errors
    /// 
    /// `CustomError` - Error personalizado
    pub fn iniciar_juego(&mut self, server: &mut Server) -> Result<(), CustomError> {
        while !self.finalizo() {
            let jugador_actual = &mut self.jugadores[self.turno];
            println!("Turno del jugador {}", jugador_actual.id);
            if let Some(conexion) = server.conexiones_jugadores.get(&jugador_actual.id) {
                let mut conexion = conexion.lock().unwrap();
                let mensaje_serializado = serde_json::to_string(&Mensaje::RealiceAccion).unwrap();
                Self::enviar_mensaje(&mut conexion, mensaje_serializado.as_bytes().to_vec())?;
            }
            jugador_actual.manejar_turno(server);
            
            loop {
                match server.recibir_mensaje(jugador_actual.id) {
                    Ok(mensaje_serializado) => {
                        match serde_json::from_str::<Mensaje>(&mensaje_serializado) {
                            Ok(mensaje) => {
                                if let Mensaje::Accion(instruccion) = mensaje {
                                    if let Some(conexion) = server.conexiones_jugadores.get(&jugador_actual.id) {
                                        let mut conexion = conexion.lock().unwrap();
                                        let jugadores = server.juego.jugadores.clone();
                                        match Self::manejar_instruccion(instruccion, jugador_actual, &mut conexion, jugadores) {
                                            Ok(_) => break, // Salir del loop si la instrucción se maneja correctamente
                                            Err(e) => {
                                                println!("Error al manejar la instrucción: {}", e);
                                                // Aquí puedes decidir si quieres seguir intentando o romper el loop
                                                // Si decides seguir intentando, el loop continuará y se intentará recibir otro mensaje
                                            }
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                println!("Error al deserializar el mensaje: {}", err);
                                break; // Salir del loop si hay un error de deserialización
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error al recibir mensaje del cliente: {:?}", e);
                        break; // Salir del loop si hay un error al recibir el mensaje
                    }
                }
            }
    
            self.turno = (self.turno + 1) % self.jugadores.len();
        }
        Ok(())
    }
    

    fn manejar_instruccion(instruccion: Instruccion, jugador_actual: &mut Jugador, conexion: &mut MutexGuard<'_, TcpStream>, jugadores: Vec<Jugador>) -> Result<(), CustomError> {
        match instruccion {
            Instruccion::Movimiento(barco_id, cordenadas) => {
                let barco = jugador_actual.obtener_barco(barco_id);
                Self::procesar_movimiento(barco, cordenadas, jugador_actual, conexion)?;
                //Self::procesar_movimiento(movimiento, &mut self.jugadores);
            }
            Instruccion::Ataque(_barco_id, coordenadas_ataque) => {
                Self::procesar_ataque(coordenadas_ataque, jugador_actual, jugadores);
            }
            Instruccion::Saltar => {
                println!("Jugador salta su turno.");
            }
            Instruccion::Tienda => {
                println!("Jugador abre la tienda");
            }
        }
        Ok(())
    }

    /// Función que verifica si el juego ha finalizado
    /// 
    /// # Returns
    /// 
    /// `bool` - Indica si el juego ha finalizado
    
    fn finalizo(&self) -> bool {
        let jugadores_con_barcos = self.jugadores.iter().filter(|j| !j.barcos.is_empty()).count();
        if jugadores_con_barcos <= 1 {
            if let Some(jugador) = self.jugadores.iter().find(|j| !j.barcos.is_empty()) {
                println!("El jugador {} ha ganado", jugador.id);
            } else {
                println!("No hay ganadores.");
            }
            return true;
        }
        false
    }

    /// Función que agrega un jugador al juego
    /// 
    /// # Args
    /// 
    /// `jugadores` - Vector de jugadores
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador agregado
    pub fn agregar_jugador(&mut self) {
        let ultimo_id = self.jugadores.len();
        self.jugadores.push(Jugador::new(ultimo_id, &mut self.mapa));
    }
    /// Función que procesa un movimiento en el mapa
    /// 
    /// # Args
    /// 
    /// `movimiento` - Movimiento a procesar
    /// 
    /// `jugadores` - Vector de jugadores
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador con el movimiento procesado
    fn procesar_movimiento(barco: Barco, cordenadas: (i32, i32), jugador_actual: &mut Jugador, conexion: &mut MutexGuard<'_, TcpStream>) -> Result<(), CustomError> {
        if barco.estado == EstadoBarco::Golpeado || barco.estado == EstadoBarco::Hundido {
            let mensaje = "El barco seleccionado esta golpeado, no se puede mover, elija otra accion u otro barco.";
            let mensaje_serializado = serde_json::to_string(&Mensaje::RepetirAccion(mensaje.to_owned(), jugador_actual.mapa.serializar_barcos(&jugador_actual.barcos))).unwrap();
            Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec())?;
            return Err(AccionInvalida)
        }
        let coordenadas_contiguas = jugador_actual.mapa.obtener_coordenadas_contiguas(cordenadas,barco.tamaño);
        if coordenadas_contiguas.is_empty() {
            let mensaje = "No hay suficientes espacios contiguos disponibles para mover el barco.";
            let mensaje_serializado = serde_json::to_string(&Mensaje::RepetirAccion(mensaje.to_owned(),jugador_actual.mapa.serializar_barcos(&jugador_actual.barcos))).unwrap();
            Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec())?;
            return Err(AccionInvalida)
        }
        let mut nuevas_posiciones = vec![];
        for (i, &coordenada) in coordenadas_contiguas.iter().enumerate() {
            nuevas_posiciones.push(coordenada);
            if i == barco.tamaño - 1 {
                break;
            }
        }
        jugador_actual.mapa.actualizar_posicion_barco(&barco.posiciones, &nuevas_posiciones, jugador_actual.id);

        Ok(())
    }
    /// Función que procesa un ataque en el mapa
    /// 
    /// # Args
    /// 
    /// `coordenadas_ataque` - Coordenadas del ataque
    /// 
    /// `jugador_id` - ID del jugador que realiza el ataque
    /// 
    /// `jugadores` - Vector de jugadores
    /// 
    /// `mapa` - Mapa en el que se realiza el ataque
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador con el ataque procesado
    fn procesar_ataque(coordenadas_ataque: (i32, i32), jugador_actual: &mut Jugador, mut jugadores: Vec<Jugador>) {
        let mut puntos_ganados = 0;
        for jugador in jugadores.iter_mut() {
            let mut jugador_atacado = jugador.clone();
            if jugador.id != jugador_actual.id {
                let puntos = jugador.procesar_ataque(coordenadas_ataque, &mut jugador_atacado.mapa);
                puntos_ganados += puntos;
                if puntos > 0 {
                    jugador.mapa.marcar_hundido(coordenadas_ataque);
                }
            
            }

        }
        jugador_actual.puntos += puntos_ganados;
    }

    fn enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }
}
