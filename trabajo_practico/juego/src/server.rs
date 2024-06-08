use libreria::{constantes::PREMIO, custom_error::CustomError};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::{juego::Juego, jugador::Jugador, mensaje::Mensaje};

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
            self_clone.preguntar_comienzo_juego();
        });

        self.jugadores.lock().unwrap().push(handle);

        Ok(())
    }

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

    pub fn enviar_mensaje(stream: &mut TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }

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

    fn esperar_jugadores(&self) {
        for connection in self.conexiones_jugadores.values() {
            let mut connection = connection.lock().unwrap();
            let mensaje_serializado = serde_json::to_string(&Mensaje::Esperando).unwrap();
            Server::enviar_mensaje(&mut connection, mensaje_serializado.as_bytes().to_vec())
                .unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn preguntar_comienzo_juego(&self) {
        if self.conexiones_jugadores.len() < 2 {
            println!("Esperando más jugadores para comenzar el juego...");
            self.esperar_jugadores();
        } else {
            let mut respuestas: HashMap<usize, String> = HashMap::new();

            for connection in self.conexiones_jugadores.values() {
                let mut connection = connection.lock().unwrap();
                let mensaje_serializado =
                    serde_json::to_string(&Mensaje::PreguntaComienzo).unwrap();
                Server::enviar_mensaje(&mut connection, mensaje_serializado.as_bytes().to_vec())
                    .unwrap();
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
                self.comenzar_juego();
            } else {
                println!("Al menos un jugador no quiere comenzar el juego. Esperando nuevas conexiones...");
            }
        }
    }
    pub fn comenzar_juego(&self) {
        let mut self_clone = self.clone();
        let mut self_clone_dos = self.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            self_clone.juego.iniciar_juego(&mut self_clone_dos).unwrap();
        });
        self.jugadores.lock().unwrap().push(handle);
    }

    pub fn crear_evento_sorpresa(&mut self, jugadores: &mut [Jugador]) {
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
                jugadores[*player_id].puntos += PREMIO;
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
    }
}
