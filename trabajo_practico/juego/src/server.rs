use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use libreria::custom_error::CustomError;

use crate::juego::Juego;

#[derive(Clone)]
pub struct Server {
    arc_server: Arc<TcpListener>,
    jugadores: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    pub conexiones_jugadores: HashMap<usize, Arc<Mutex<TcpStream>>>,
    juego: Juego,
    next_player_id: usize,
    jugadores_conectados: usize,
}

impl Server {
    pub fn new() -> Result<Self, CustomError> {
        let server = TcpListener::bind("127.0.0.1:8080").map_err(|_| CustomError::ErrorCreatingSocket)?;
        let jugadores = Arc::new(Mutex::new(Vec::new()));
        let conexiones_jugadores = HashMap::new();
        let juego = Juego::new(0);
        println!("Servidor iniciado.");
        Ok(Server {
            arc_server: Arc::new(server),
            jugadores,
            conexiones_jugadores,
            juego,
            next_player_id: 0,
            jugadores_conectados: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), CustomError> {
        let mut self_clone = self.clone();
        for stream in self.arc_server.incoming() {
            let stream = stream.map_err(|_| CustomError::ErrorAceptandoConexion)?;
            self_clone.jugadores_conectados += 1;
            println!("Nuevo jugador conectado");
            self_clone.handle_client(stream)?;
            
        }
        Ok(())
    }

    fn handle_client(&mut self, stream: TcpStream) -> Result<(), CustomError> {
        // Asigna un ID único al jugador
        let player_id = self.next_player_id;
        self.next_player_id += 1;

        let player_connection = Arc::new(Mutex::new(stream));
        self.conexiones_jugadores.insert(player_id, player_connection.clone());

        let mensaje_registro = format!(
            "Registrado correctamente con el id '{}', esperando jugadores para iniciar el juego.",
            player_id
        );
        Server::enviar_mensaje(&mut player_connection.lock().unwrap(), mensaje_registro.as_bytes().to_vec())?;

        self.juego.agregar_jugador();
        let self_clone = self.clone();
        let handle = thread::spawn(move || {
            self_clone.preguntar_comienzo_juego();
        });

        self.jugadores.lock().unwrap().push(handle);

        Ok(())
    }

    pub fn enviar_instruccion(&self, player_id: usize, instruccion: &str) -> Result<(), CustomError> {
        if let Some(player_connection) = self.conexiones_jugadores.get(&player_id) {
            let mut connection = player_connection.lock().unwrap();
            connection.write_all(instruccion.as_bytes()).map_err(|_| CustomError::ErrorEnviandoInstruccion)?;
            Ok(())
        } else {
            Err(CustomError::ErrorJugadorInexistente)
        }
    }

    fn enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }

    fn esperar_jugadores(&self) {
        let mensaje_espera = "Esperando mas jugadores para comenzar el juego...";
        for (_, connection) in &self.conexiones_jugadores {
            let mut connection = connection.lock().unwrap();
            Server::enviar_mensaje(&mut connection, mensaje_espera.as_bytes().to_vec()).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn preguntar_comienzo_juego(&self) {
        if self.conexiones_jugadores.len() <2 {
            println!("Esperando más jugadores para comenzar el juego...");
            self.esperar_jugadores();
        } else {
            let mensaje_pregunta = "¿Desean comenzar el juego? (si/no)";
            let mut respuestas: HashMap<usize, String> = HashMap::new();
            
            for (_, connection) in &self.conexiones_jugadores {
                let mut connection = connection.lock().unwrap();
                Server::enviar_mensaje(&mut connection, mensaje_pregunta.as_bytes().to_vec()).unwrap();
            }
            
            for (player_id, connection) in &self.conexiones_jugadores {
                let mut connection = connection.lock().unwrap();
                let mut buffer = [0; 512];
                let bytes_read = connection.read(&mut buffer).unwrap();
                let respuesta = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
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
    



}
