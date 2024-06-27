use libreria::{
    constantes::{CORDENADAS_BOMBA, PREMIO},
    custom_error::CustomError,
};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::{juego::Juego, jugador::Jugador, mensaje::Mensaje};

/// Estructura que representa el servidor

#[derive(Clone)]
pub struct Server {
    arc_server: Arc<TcpListener>,
    jugadores: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    pub conexiones_jugadores: HashMap<usize, Arc<Mutex<TcpStream>>>,
    nombres_jugadores: HashMap<usize, String>,
    pub juego: Juego,
    next_player_id: usize,
    jugadores_conectados: usize,
}

impl Server {
    /// Función que crea un nuevo servidor
    ///
    /// # Returns
    ///
    /// `Result<Self, CustomError>` - Resultado de la función
    ///
    /// # Errors
    ///
    /// `CustomError::ErrorCreatingSocket` - Error al crear el socket
    pub fn new() -> Result<Self, CustomError> {
        let server =
            TcpListener::bind("127.0.0.1:8080").map_err(|_| CustomError::ErrorCreatingSocket)?;
        let jugadores = Arc::new(Mutex::new(Vec::new()));
        let conexiones_jugadores = HashMap::new();
        let nombres_jugadores = HashMap::new();
        let juego = Juego::new(0);
        println!("Servidor iniciado.");
        Ok(Server {
            arc_server: Arc::new(server),
            jugadores,
            conexiones_jugadores,
            nombres_jugadores,
            juego,
            next_player_id: 0,
            jugadores_conectados: 0,
        })
    }
    /// Función que ejecuta el servidor
    ///
    /// # Returns
    ///
    /// `Result<(), CustomError>` - Resultado de la función
    ///
    /// # Errors
    ///
    /// `CustomError::ErrorAceptandoConexion` - Error al aceptar la conexión
    ///
    /// `CustomError::ErrorRecibiendoInstruccion` - Error al recibir la instrucción
    pub fn run(&mut self) -> Result<(), CustomError> {
        let mut self_clone = self.clone();
        for stream in self.arc_server.incoming() {
            let mut stream = stream.map_err(|_| CustomError::ErrorAceptandoConexion)?;
            self_clone.jugadores_conectados += 1;
            println!("Nuevo jugador conectado");

            let mensaje_serializado = serde_json::to_string(&Mensaje::Registro).unwrap();
            Self::enviar_mensaje(&mut stream, mensaje_serializado.as_bytes().to_vec()).unwrap();

            let mut buffer = [0; 2048];
            let bytes_read = stream
                .read(&mut buffer)
                .map_err(|_| CustomError::ErrorRecibiendoInstruccion)?;
            let nombre_usuario = String::from_utf8_lossy(&buffer[..bytes_read])
                .trim()
                .to_string();
            println!(
                "Jugador conectado con el nombre de usuario: {}",
                nombre_usuario
            );

            self_clone.handle_client(stream, nombre_usuario)?;
        }
        Ok(())
    }
    /// Función que maneja al cliente
    ///
    /// # Args
    ///
    /// `stream` - Flujo de datos
    ///
    /// `nombre_usuario` - Nombre del usuario
    ///
    /// # Returns
    ///
    /// `Result<(), CustomError>` - Resultado de la función
    fn handle_client(
        &mut self,
        stream: TcpStream,
        nombre_usuario: String,
    ) -> Result<(), CustomError> {
        let jugador_id = self.next_player_id;
        self.next_player_id += 1;
        let player_connection = Arc::new(Mutex::new(stream));
        self.conexiones_jugadores
            .insert(jugador_id, player_connection);
        self.nombres_jugadores
            .insert(jugador_id, nombre_usuario.clone());
        self.juego
            .agregar_jugador(jugador_id, nombre_usuario.clone());
        let self_clone = self.clone();
        let handle = thread::spawn(move || {
            let _ = self_clone
                .preguntar_comienzo_juego()
                .map_err(|_| CustomError::ErrorThreads);
        });

        self.jugadores.lock().unwrap().push(handle);

        Ok(())
    }
    /// Función que envía una instrucción
    ///
    /// # Args
    ///
    /// `player_id` - ID del jugador
    ///
    /// `instruccion` - Instrucción
    ///
    /// # Returns
    ///
    /// `Result<(), CustomError>` - Resultado de la función
    pub fn enviar_instruccion(
        &self,
        player_id: usize,
        instruccion: &str,
    ) -> Result<(), CustomError> {
        if let Some(player_connection) = self.conexiones_jugadores.get(&player_id) {
            let mut connection = player_connection.lock().unwrap();
            connection
                .write_all(instruccion.as_bytes())
                .map_err(|_| CustomError::ErrorEnviandoInstruccion)?;
            Ok(())
        } else {
            Err(CustomError::ErrorJugadorInexistente)
        }
    }
    /// Función que envía un mensaje
    ///
    /// # Args
    ///
    /// `stream` - Flujo de datos
    ///
    /// `msg` - Mensaje
    ///
    /// # Returns
    ///
    /// `Result<(), CustomError>` - Resultado de la función
    pub fn enviar_mensaje(stream: &mut TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }
    /// Función que recibe un mensaje
    ///
    /// # Args
    ///
    /// `id` - ID del jugador
    ///
    /// # Returns
    ///
    /// `Result<String, CustomError>` - Resultado de la función
    pub fn recibir_mensaje(&mut self, id: usize) -> Result<String, CustomError> {
        let mut buffer = [0; 2048];
        let stream = self.conexiones_jugadores.get(&id).unwrap();
        let bytes_read = stream
            .lock()
            .unwrap()
            .read(&mut buffer)
            .map_err(|_| CustomError::ErrorRecibiendoInstruccion)?;
        let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(message)
    }
    /// Función que espera a los jugadores
    ///
    /// `()` - No retorna nada
    fn esperar_jugadores(&self) {
        for connection in self.conexiones_jugadores.values() {
            let mut connection = connection.lock().unwrap();
            let mensaje_serializado = serde_json::to_string(&Mensaje::Esperando).unwrap();
            Server::enviar_mensaje(&mut connection, mensaje_serializado.as_bytes().to_vec())
                .unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    /// Función que pregunta si se quiere comenzar el juego
    ///
    /// # Returns
    ///
    /// `Result<(), CustomError>` - Ok si se puede comenzar el juego o Error si no se puede
    pub fn preguntar_comienzo_juego(&self) -> Result<(), CustomError> {
        if self.conexiones_jugadores.len() < 2 {
            println!("Esperando más jugadores para comenzar el juego...");
            self.esperar_jugadores();
            Ok(())
        } else {
            let mut respuestas: HashMap<usize, String> = HashMap::new();

            for connection in self.conexiones_jugadores.values() {
                let mut connection = connection.lock().unwrap();
                let mensaje_serializado =
                    serde_json::to_string(&Mensaje::PreguntaComienzo).unwrap();
                Server::enviar_mensaje(&mut connection, mensaje_serializado.as_bytes().to_vec())
                    .map_err(|_| CustomError::ErrorEnviarMensaje)?;
            }

            for (player_id, connection) in &self.conexiones_jugadores {
                let mut connection = connection.lock().unwrap();
                let mut buffer = [0; 512];
                let bytes_read = connection.read(&mut buffer).unwrap();
                let respuesta = String::from_utf8_lossy(&buffer[..bytes_read])
                    .trim()
                    .to_string();
                respuestas.insert(*player_id, respuesta);
            }
            if respuestas.values().all(|respuesta| respuesta == "si") {
                println!("Todos los jugadores quieren comenzar el juego.");
                println!("Comenzando el juego...");
                let _ = self.comenzar_juego();
                Ok(())
            } else {
                println!("Al menos un jugador no quiere comenzar el juego. Esperando nuevas conexiones...");
                let _ = self.preguntar_comienzo_juego();

                Ok(())
            }
        }
    }
    /// Función que comienza el juego
    ///
    /// # Returns
    ///
    /// `Result<(), CustomError>` - Resultado de la función
    ///
    /// # Errors
    ///
    /// `CustomError::ErrorThreads` - Error en los threads
    pub fn comenzar_juego(&self) -> Result<(), CustomError> {
        let mut self_clone = self.clone();
        let mut self_clone_dos = self.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let _ = self_clone
                .juego
                .iniciar_juego(&mut self_clone_dos)
                .map_err(|_| CustomError::ErrorThreads);
        });
        self.jugadores.lock().unwrap().push(handle);
        Ok(())
    }
    /// Función que crea un evento sorpresa
    ///
    /// # Args
    ///
    /// `jugadores` - Vector de jugadores
    ///
    /// # Returns
    ///
    /// `()` - No retorna nada
    pub fn crear_evento_sorpresa(&mut self, jugadores: &mut [Jugador]) {
        //esperar 2 segundos
        std::thread::sleep(std::time::Duration::from_secs(2));
        for connection in self.conexiones_jugadores.values() {
            let mut connection = connection.lock().unwrap();
            let mensaje_serializado = serde_json::to_string(&Mensaje::EventoSorpresa).unwrap();
            Server::enviar_mensaje(&mut connection, mensaje_serializado.as_bytes().to_vec())
                .unwrap();
        }
    
        let mut primero = None;
    
        for (player_id, connection) in &self.conexiones_jugadores {
            let mut connection = connection.lock().unwrap();
            let mut buffer = [0; 512];
            let bytes_read = connection.read(&mut buffer).unwrap();
            let respuesta = String::from_utf8_lossy(&buffer[..bytes_read])
                .trim()
                .to_string();
    
            if respuesta == "primero" && primero.is_none() {
                primero = Some(*player_id);
                jugadores[*player_id].monedas += PREMIO;
                let mensaje_especial =
                    serde_json::to_string(&Mensaje::EventoSorpresaResultado(true)).unwrap();
                Server::enviar_mensaje(&mut connection, mensaje_especial.as_bytes().to_vec())
                    .unwrap();
            } else {
                let mensaje_especial =
                    serde_json::to_string(&Mensaje::EventoSorpresaResultado(false)).unwrap();
                Server::enviar_mensaje(&mut connection, mensaje_especial.as_bytes().to_vec())
                    .unwrap();
            }
        }
    
        if let Some(p) = primero {
            let posiciones = CORDENADAS_BOMBA;
            for posicion in posiciones {
                let self_clone = self.clone();
                let mut conexion = self_clone
                    .conexiones_jugadores
                    .get(&p)
                    .unwrap()
                    .lock()
                    .unwrap();
                Juego::procesar_ataque(*posicion, p, jugadores, &self_clone, &mut conexion);
            }
        }
    }
    
}
