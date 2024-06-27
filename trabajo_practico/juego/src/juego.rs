use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::{io::Write, net::TcpStream, sync::MutexGuard};

use crate::juego::CustomError::AccionInvalida;
use crate::{
    jugador::Jugador,
    mapa::Mapa,
    mensaje::{Instruccion, Mensaje},
    server::Server,
};
use barcos::estado_barco::EstadoBarco;
use libreria::constantes::EVENTO_SORPRESA;
use libreria::custom_error::CustomError;

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
            jugadores.push(Jugador::new(jugadores.len(), "".to_string(), &mut mapa));
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
        let mut rondas = 0;

        while let Ok(finalizado) = self.finalizo(server) {
            if finalizado {
                break;
            }

            if rondas == EVENTO_SORPRESA {
                server.crear_evento_sorpresa(&mut self.jugadores);
            }

            println!(
                "Turno del jugador {}",
                self.jugadores[self.turno].nombre_usuario
            );

            if let Some(conexion) = server
                .conexiones_jugadores
                .get(&self.jugadores[self.turno].id)
            {
                let conexion = conexion.lock().unwrap();
                let mensaje_serializado = serde_json::to_string(&Mensaje::RealiceAccion).unwrap();
                Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec())?;
            }

            self.jugadores[self.turno].manejar_turno(server);

            loop {
                match server.recibir_mensaje(self.jugadores[self.turno].id) {
                    Ok(mensaje_serializado) => {
                        match serde_json::from_str::<Mensaje>(&mensaje_serializado) {
                            Ok(mensaje) => {
                                if let Mensaje::Accion(instruccion) = mensaje {
                                    if let Some(conexion) = server
                                        .conexiones_jugadores
                                        .get(&self.jugadores[self.turno].id)
                                    {
                                        let mut conexion = conexion.lock().unwrap();
                                        match Self::manejar_instruccion(
                                            instruccion,
                                            self.turno,
                                            &mut conexion,
                                            &mut self.jugadores,
                                            server,
                                        ) {
                                            Ok(_) => break,
                                            Err(_) => {
                                                return Err(
                                                    CustomError::ErrorRecibiendoInstruccion,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                return Err(CustomError::ErrorSerializacion);
                            }
                        }
                    }
                    Err(_) => {
                        return Err(CustomError::ErrorAceptandoConexion);
                    }
                }
            }

            self.jugadores[self.turno].enviar_instrucciones(server);
            self.turno = (self.turno + 1) % self.jugadores.len();
            rondas += 1;
        }

        match self.finalizo(server) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn mostrar_ranking(conexion: &mut MutexGuard<'_, TcpStream>) -> Result<(), CustomError> {
        let file_path = "../archivos/ranking.json";
        let file = File::open(file_path).map_err(|_| CustomError::ErrorMostrandoRanking)?;
        let reader = BufReader::new(file);

        let rankings: HashMap<String, usize> = serde_json::from_reader(reader).unwrap_or_default();

        let mut ranking_sorted: Vec<_> = rankings.iter().collect();
        ranking_sorted.sort_by_key(|&(_, puntos)| std::cmp::Reverse(*puntos));

        let ranking_vec: Vec<(String, usize)> = ranking_sorted
            .into_iter()
            .map(|(nombre, puntos)| (nombre.clone(), *puntos))
            .collect();

        let mensaje = Mensaje::Ranking(ranking_vec);

        let mensaje_serializado =
            serde_json::to_string(&mensaje).map_err(|_| CustomError::ErrorSerializacion)?;

        Self::enviar_mensaje(conexion, mensaje_serializado.into_bytes())
    }

    fn manejar_instruccion(
        instruccion: Instruccion,
        jugador_actual: usize,
        conexion: &mut MutexGuard<'_, TcpStream>,
        jugadores:&mut [Jugador],
        server: &Server,
    ) -> Result<(), CustomError> {
        match instruccion {
            Instruccion::Movimiento(barco_id, cordenadas) => {
                Self::procesar_movimiento(
                    barco_id,
                    cordenadas,
                    jugador_actual,
                    jugadores,
                    conexion,
                )?;
                //Self::procesar_movimiento(movimiento, &mut self.jugadores);
            }
            Instruccion::Ataque(_barco_id, coordenadas_ataque) => {
                Self::procesar_ataque(coordenadas_ataque, jugador_actual, jugadores, server,conexion);
            }
            Instruccion::Saltar => {
                println!("Jugador salta su turno.");
            }
            Instruccion::Compra(barco_elegido) => {
                Self::abrir_tienda(jugadores, jugador_actual, barco_elegido);
            }
            Instruccion::Ranking => {
                if Self::mostrar_ranking(conexion).is_err(){
                    return Err(CustomError::ErrorMostrandoRanking);
                }
            }
        }
        Ok(())
    }

    /// Función que verifica si el juego ha finalizado
    ///
    /// # Returns
    ///
    /// `bool` - Indica si el juego ha finalizado

    fn finalizo(&self,server: &Server) -> Result<bool, CustomError> {
        let jugadores_con_barcos = self
            .jugadores
            .iter()
            .filter(|j| !j.barcos.is_empty())
            .count();
        if jugadores_con_barcos <= 1 {
            if let Some(jugador) = self.jugadores.iter().find(|j| !j.barcos.is_empty()) {
                self.actualizar_ranking()
                    .map_err(|_| CustomError::ErrorMostrandoRanking)?;
                println!("El jugador {} ha ganado", jugador.nombre_usuario);
                let mensaje_serializado = serde_json::to_string(&Mensaje::FinPartida(
                    jugador.nombre_usuario.clone(),
                    jugador.puntos,
                ));
                if let Ok(mensaje) = mensaje_serializado {
                    for (_, conexion) in &server.conexiones_jugadores {
                        let mut conexion = conexion.lock().unwrap();
                        Self::enviar_mensaje(&mut conexion, mensaje.as_bytes().to_vec())?;
                    }
                }
            } else {
                println!("No hay ganadores.");
            }
            return Ok(true);
        }
        Ok(false)
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
    pub fn agregar_jugador(&mut self, id_jugador: usize, nombre: String) {
        self.jugadores
            .push(Jugador::new(id_jugador, nombre, &mut self.mapa));
    }

    fn abrir_tienda(jugadores: &mut [Jugador], jugador_actual: usize, barco: usize) {
        jugadores[jugador_actual].agregar_barco(barco);
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
    fn procesar_movimiento(
        barco_id: usize,
        cordenadas: (i32, i32),
        jugador_actual: usize,
        jugadores:&mut [Jugador],
        conexion: &mut MutexGuard<'_, TcpStream>,
    ) -> Result<(), CustomError> {
        let barco = jugadores[jugador_actual].obtener_barco(barco_id);
        if barco.estado == EstadoBarco::Golpeado || barco.estado == EstadoBarco::Hundido {
            let mensaje = "El barco seleccionado esta golpeado, no se puede mover, elija otra accion u otro barco.";
            let mensaje_serializado = serde_json::to_string(&Mensaje::RepetirAccion(
                mensaje.to_owned(),
                jugadores[jugador_actual]
                    .mapa
                    .serializar_barcos(&jugadores[jugador_actual].barcos),
                jugadores[jugador_actual].monedas.clone()
            ))
            .unwrap();
            Self::enviar_mensaje(conexion, mensaje_serializado.as_bytes().to_vec())?;
            return Err(AccionInvalida);
        }
        let coordenadas_contiguas = jugadores[jugador_actual]
            .mapa
            .obtener_coordenadas_contiguas(cordenadas, barco.tamaño);
        if coordenadas_contiguas.is_empty() {
            let mensaje = "No hay suficientes espacios contiguos disponibles para mover el barco.";
            let mensaje_serializado = serde_json::to_string(&Mensaje::RepetirAccion(
                mensaje.to_owned(),
                jugadores[jugador_actual]
                    .mapa
                    .serializar_barcos(&jugadores[jugador_actual].barcos),
                jugadores[jugador_actual].monedas.clone()
            ))
            .unwrap();
            Self::enviar_mensaje(conexion, mensaje_serializado.as_bytes().to_vec())?;
            return Err(AccionInvalida);
        }

        jugadores[jugador_actual].actualizar_posicion_barco(coordenadas_contiguas, barco_id);

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
    pub fn procesar_ataque(
        coordenadas_ataque: (i32, i32),
        jugador_actual: usize,
        jugadores: &mut [Jugador],
        server: &Server,
        conexion: &mut MutexGuard<'_, TcpStream>
    ) {
        let mut puntos_ganados = 0;
        let mut monedas_ganadas = 0;
        for jugador in jugadores.iter_mut() {
            if jugador.id != jugador_actual {
                let (puntos,monedas) = jugador.procesar_ataque(coordenadas_ataque,server);
                puntos_ganados += puntos;
                monedas_ganadas += monedas;
                if puntos > 0 {
                    jugador.mapa.marcar_hundido(coordenadas_ataque);
                }
            }
        }
        let mensaje = Mensaje::MensajeInfoAaque(puntos_ganados, monedas_ganadas);
        let mensaje_serializado = serde_json::to_string(&mensaje).unwrap();
        Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec()).unwrap();
        jugadores[jugador_actual].puntos += puntos_ganados;
        jugadores[jugador_actual].monedas += monedas_ganadas;
    }
    /// Función que envía un mensaje a un cliente
    /// 
    /// # Args
    /// 
    /// `stream` - Stream de conexión   
    /// 
    /// `msg` - Mensaje a enviar
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la ejecución
    /// 
    /// # Errors
    /// 
    /// `CustomError` - Error personalizado
    fn enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }

    /// Función que actualiza el ranking de los jugadores
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la ejecución
    /// 
    /// # Errors
    /// 
    /// `CustomError` - Error personalizado
    pub fn actualizar_ranking(&self) -> Result<(), CustomError> {
        let dir_archivo: &str = "../archivos";
        let nombre_archivo = format!("{}/ranking.json", dir_archivo);

        fs::create_dir_all(dir_archivo).map_err(|_| CustomError::ErrorMostrandoRanking)?;

        let mut rankings: HashMap<String, usize> = HashMap::new();
        if let Ok(archivo) = File::open(&nombre_archivo) {
            let reader = BufReader::new(archivo);
            let ranking: HashMap<String, usize> =
                serde_json::from_reader(reader).unwrap_or_default();
            rankings.extend(ranking);
        }

        for jugador in &self.jugadores {
            let entry = rankings.entry(jugador.nombre_usuario.clone()).or_insert(0);
            *entry += jugador.puntos;
        }

        let mut ranking_ordenado: Vec<_> = rankings.iter().collect();
        ranking_ordenado.sort_by_key(|&(_, puntos)| std::cmp::Reverse(*puntos));

        let ranking_final: HashMap<_, _> = ranking_ordenado
            .into_iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(nombre_archivo)
            .map_err(|_| CustomError::ErrorMostrandoRanking)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &ranking_final)
            .map_err(|_| CustomError::ErrorSerializacion)?;

        Ok(())
    }
}
