use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::Once;
use std::{io::Write, net::TcpStream, sync::MutexGuard};

use crate::juego::CustomError::AccionInvalida;
use crate::{
    instruccion::Instruccion, jugador::Jugador, mapa::Mapa, mensaje::Mensaje, server::Server,
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
        let mut server_clone = server.clone();
    
        while self.finalizo().is_ok(){
    
            let jugadores_con_barcos: Vec<_> = self.jugadores.iter().filter(|j| !j.barcos.is_empty()).collect();
            if jugadores_con_barcos.len() == 1 {
                println!("El ganador es: {}", jugadores_con_barcos[0].nombre_usuario);
                let mensaje = Mensaje::Ganaste(jugadores_con_barcos[0].puntos);
                let mensaje_serializado = serde_json::to_string(&mensaje).unwrap();
                for (id, conexion) in &server_clone.conexiones_jugadores {
                    if *id == jugadores_con_barcos[0].id {
                        let conexion = conexion.lock().unwrap();
                        let _ = Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec());
                    }
                }
                return Ok(());
            }
    
            if rondas == EVENTO_SORPRESA {
                let _ = server_clone.crear_evento_sorpresa(&mut self.jugadores);
            }
    
            while self.jugadores[self.turno].barcos.is_empty() {
                self.turno = (self.turno + 1) % self.jugadores.len();
            }
    
            println!(
                "Turno del jugador {}",
                self.jugadores[self.turno].nombre_usuario
            );
            println!(
                "Cantidad de jugadores con barcos: {:?}",
                self.jugadores
                    .iter()
                    .filter(|j| !j.barcos.is_empty())
                    .count()
            );
    
            let mut server_clone = server_clone.clone();
            if let Some(conexion) = server_clone
                .conexiones_jugadores
                .get(&self.jugadores[self.turno].id)
            {
                let conexion = conexion.lock().unwrap();
                let mensaje_serializado = serde_json::to_string(&Mensaje::RealiceAccion).unwrap();
                Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec())?;
            }
    
            self.jugadores[self.turno].manejar_turno(&server_clone);
    
            loop {
                match server_clone.recibir_mensaje(self.jugadores[self.turno].id) {
                    Ok(mensaje_serializado) => {
                        match serde_json::from_str::<Mensaje>(&mensaje_serializado) {
                            Ok(mensaje) => {
                                if let Mensaje::Accion(instruccion, monedas) = mensaje {
                                    if let Some(conexion) = server_clone
                                        .conexiones_jugadores
                                        .get(&self.jugadores[self.turno].id)
                                    {
                                        let mut conexion = conexion.lock().unwrap();
                                        let mut server_mut = server_clone.clone();
                                        let mut self_clone = self.clone();
                                        match self_clone.manejar_instruccion(
                                            instruccion,
                                            self.turno,
                                            &mut conexion,
                                            &mut self.jugadores,
                                            &mut server_mut,
                                            monedas,
                                        ) {
                                            Ok(_) => break,
                                            Err(_) => {
                                                return Err(CustomError::ErrorRecibiendoInstruccion);
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
    
            self.jugadores[self.turno].enviar_instrucciones(&server_clone);
            self.turno = (self.turno + 1) % self.jugadores.len();
            rondas += 1;
        }
    
        match self.finalizo() {
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
    /// Función que maneja una instrucción
    /// 
    /// # Args
    /// 
    /// `instruccion` - Instrucción a manejar
    /// 
    /// `jugador_actual` - Jugador que realiza la instrucción
    /// 
    /// `conexion` - Conexión del jugador
    /// 
    /// `jugadores` - Vector de jugadores
    /// 
    /// `server` - Servidor en el que se encuentra el juego
    /// 
    /// `monedas` - Monedas del jugador
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la ejecución
    fn manejar_instruccion(
        &mut self,
        instruccion: Instruccion,
        jugador_actual: usize,
        conexion: &mut MutexGuard<'_, TcpStream>,
        jugadores: &mut [Jugador],
        server: &mut Server,
        monedas: usize,
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
            }
            Instruccion::Ataque(_barco_id, coordenadas_ataque) => {
                let pierde = Self::procesar_ataque(
                    coordenadas_ataque,
                    jugador_actual,
                    jugadores,
                    server,
                    conexion,
                );
                if pierde {
                    self.eliminar_jugador(jugadores[jugador_actual].id);
                    server
                        .conexiones_jugadores
                        .remove(&jugadores[jugador_actual].id);
                }
            }

            Instruccion::Saltar => {
                println!("Jugador salta su turno.");
            }
            Instruccion::Compra(barco_elegido) => {
                Self::abrir_tienda(jugadores, jugador_actual, barco_elegido);
                jugadores[jugador_actual].monedas -= monedas;
                let mensaje =
                    Mensaje::CompraExitosa(jugadores[jugador_actual].monedas, barco_elegido);
                let mensaje_serializado = serde_json::to_string(&mensaje).unwrap();
                Self::enviar_mensaje(conexion, mensaje_serializado.as_bytes().to_vec()).unwrap();
                match barco_elegido {
                    0 => {
                        println!(
                            "El jugador {} ha comprado una fragata",
                            jugadores[jugador_actual].nombre_usuario
                        );
                    }
                    1 => {
                        println!(
                            "El jugador {} ha comprado un buque",
                            jugadores[jugador_actual].nombre_usuario
                        );
                    }
                    2 => {
                        println!(
                            "El jugador {} ha comprado un acorazado",
                            jugadores[jugador_actual].nombre_usuario
                        );
                    }
                    _ => {}
                }
            }
            Instruccion::Ranking => {
                if Self::mostrar_ranking(conexion).is_err() {
                    return Err(CustomError::ErrorMostrandoRanking);
                }
            }
        }
        Ok(())
    }

    /// Función que finaliza el juego
    ///
    /// # Args
    ///
    /// `server` - Servidor en el que se encuentra el juego
    ///
    /// # Returns
    ///
    /// `Result<bool, CustomError>` - Result con booleano que indica si el juego finalizó o error
    fn finalizo(&self) -> Result<bool, CustomError> {
        static ONCE_FLAG: Once = Once::new();

        let jugadores_con_barcos = self
            .jugadores
            .iter()
            .filter(|j| !j.barcos.is_empty())
            .count();

        if jugadores_con_barcos <= 1 {
            if let Some(jugador) = self.jugadores.iter().find(|j| !j.barcos.is_empty()) {
                self.actualizar_ranking()
                    .map_err(|_| CustomError::ErrorMostrandoRanking)?;

                let _ganador = &jugador.nombre_usuario;
                

                ONCE_FLAG.call_once(|| {
                    println!("Juego terminado");
                });
            } else {
                ONCE_FLAG.call_once(|| {
                    println!("No hay ganadores.");
                });
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
    /// Función que elimina un jugador del juego
    ///
    /// # Args
    ///
    /// `jugadores` - Vector de jugadores
    ///
    /// `id_jugador` - ID del jugador a eliminar
    ///
    /// # Returns
    ///
    /// `()` - No retorna nada
    pub fn eliminar_jugador(&mut self, id_jugador: usize) {
        self.jugadores.retain(|j| j.id != id_jugador);
    }
    /// Función que abre la tienda para un jugador
    ///
    /// # Args
    ///
    /// `jugadores` - Vector de jugadores
    ///
    /// `jugador_actual` - Jugador que abre la tienda
    ///
    /// `barco` - Barco a comprar
    ///
    /// # Returns
    ///
    /// `()` - No retorna nada
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
        jugadores: &mut [Jugador],
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
                jugadores[jugador_actual].monedas,
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
                jugadores[jugador_actual].monedas,
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
        server: &mut Server,
        conexion: &mut MutexGuard<'_, TcpStream>,
    ) -> bool {
        let mut pierde = false;
        let mut puntos_ganados = 0;
        let mut monedas_ganadas = 0;
    
        for jugador in jugadores.iter_mut() {
            if jugador.id != jugador_actual {
                let (puntos, monedas) = jugador.procesar_ataque(coordenadas_ataque, server);
                if jugador.barcos.is_empty() && !jugador.ha_perdido {
                    jugador.ha_perdido = true; 
    
                    let mensaje = Mensaje::Perdiste(jugador.puntos);
                    let mensaje_serializado = serde_json::to_string(&mensaje).unwrap();
                    for (id, conexion) in &server.conexiones_jugadores {
                        if *id == jugador.id {
                            let conexion = conexion.lock().unwrap();
                            let _ = Self::enviar_mensaje(
                                &conexion,
                                mensaje_serializado.as_bytes().to_vec(),
                            );
                        }
                    }
                    println!("El jugador {} ha sido eliminado", jugador.nombre_usuario);
                    pierde = true;
                    server.conexiones_jugadores.remove(&jugador.id);
                }
                puntos_ganados += puntos;
                monedas_ganadas += monedas;
                if puntos > 0 {
                    jugador.mapa.marcar_hundido(coordenadas_ataque);
                }
            }
            if server.conexiones_jugadores.len() == 1 {
                println!("El juego ha terminado");
                break;
            }
        }
    
        let mensaje = Mensaje::MensajeInfoAtaque(puntos_ganados, monedas_ganadas);
        let mensaje_serializado = serde_json::to_string(&mensaje).unwrap();
        Self::enviar_mensaje(conexion, mensaje_serializado.as_bytes().to_vec()).unwrap();
        jugadores[jugador_actual].puntos += puntos_ganados;
        jugadores[jugador_actual].monedas += monedas_ganadas;
    
        pierde
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
    /// Función que actualiza el ranking de jugadores
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la ejecución
    /// 
    /// # Errors
    /// 
    /// `CustomError` - Error personalizado
    pub fn actualizar_ranking(&self) -> Result<(), CustomError> {
        let nombre_archivo = "../archivos/ranking.json";

        fs::create_dir_all("../archivos").map_err(|_| CustomError::ErrorMostrandoRanking)?;

        let mut rankings: HashMap<String, usize> = if let Ok(archivo) = File::open(nombre_archivo) {
            let reader = BufReader::new(archivo);
            serde_json::from_reader(reader).unwrap_or_default()
        } else {
            HashMap::new()
        };

        for jugador in &self.jugadores {
            *rankings.entry(jugador.nombre_usuario.clone()).or_insert(0) += jugador.puntos;
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(nombre_archivo)
            .map_err(|_| CustomError::ErrorMostrandoRanking)?;

        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &rankings).map_err(|_| CustomError::ErrorSerializacion)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuevo_juego() {
        let juego = Juego::new(2);
        assert_eq!(juego.jugadores.len(), 2);
    }

    #[test]
    fn test_agregar_jugador() {
        let mut juego = Juego::new(2);
        juego.agregar_jugador(2, "Jugador 3".to_string());
        assert_eq!(juego.jugadores.len(), 3);
    }

    #[test]
    fn test_eliminar_jugador() {
        let mut juego = Juego::new(2);
        juego.eliminar_jugador(1);
        assert_eq!(juego.jugadores.len(), 1);
    }
}
