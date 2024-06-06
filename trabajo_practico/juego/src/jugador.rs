use acciones::accion::Accion;
use barcos::{barco::Barco, estado_barco::EstadoBarco};
use libreria::custom_error::CustomError;

use crate::{ mapa::Mapa, mensaje::Mensaje, server::Server}  ;
use std::{io::Write, net::TcpStream, vec};


#[derive(Clone)]
pub struct Jugador {
    pub id: usize,
    pub mapa: Mapa,
    pub barcos: Vec<Barco>,
    pub puntos: usize,
    pub monedas: usize,
}

impl Jugador {
    /// Función que crea un nuevo jugador
    /// 
    /// # Args
    /// 
    /// `id` - Identificador del jugador
    /// 
    /// `mapa` - Mapa en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador creado
    pub fn new(id: usize, mapa: &mut Mapa) -> Jugador {
        let mut barcos = Vec::new();
        let mut id_actual = 0;

        let tamaños_barcos: Vec<usize> = vec![5,4,3];
        for tamaño in tamaños_barcos {
            let vec_posiciones = mapa.obtener_posiciones_libres_contiguas(id.to_string(), tamaño);
            let id_barco = id_actual;
            id_actual += 1;
            barcos.push(Barco::new(id_barco, tamaño, vec_posiciones));
        }

        Jugador {
            id,
            barcos,
            puntos: 0,
            monedas: 500,
            mapa: mapa.clone(),
        }
    }

    pub fn enviar_instrucciones(&self, server: &Server) -> Result<(), CustomError> {    
        if let Some(conexion) = server.conexiones_jugadores.get(&self.id) {
            let conexion = conexion.lock().map_err(|_| CustomError::ErrorAceptandoConexion)?;
            let mensaje_serializado = serde_json::to_string(&Mensaje::Puntos(self.puntos)).unwrap();
            Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec())?;
        }
    
        Ok(())
    }

    pub fn manejar_turno(&mut self, server: &Server) {
        let _ = self.mapa.enviar_tablero(self.id.to_string(), server, &self.barcos);
        let _ = self.enviar_instrucciones(server);

    }
    
    /// Función que permite al jugador moverse en el tablero
    ///
    /// # Args
    /// 
    /// `mapa` - Mapa en el que se moverá el jugador
    /// 
    /// # Returns
    /// 
    /// `Accion` - Acción de movimiento realizada por el jugador

    
    pub fn obtener_barco(&self, barco_seleccionado: usize) -> Barco {
        self.barcos[barco_seleccionado].clone()
    }

    fn _abrir_tienda(&self) -> Accion {
        println!("Tienda abierta");
        Accion::Tienda(self.puntos)
    }
    /// Función que procesa un ataque realizado por un jugador
    /// 
    /// # Args
    /// 
    /// `coordenadas_ataque` - Coordenadas del ataque realizado por el jugador
    /// 
    /// `mapa` - Mapa en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// `usize` - Puntos ganados por el jugador
    
    pub fn procesar_ataque(&mut self, coordenadas_ataque: (i32, i32)) -> usize{
        let mut puntos = 0;
        let mut barcos_golpeados = false;
        let mut barcos_hundidos = Vec::new();

        for barco in &mut self.barcos {
            if barco.posiciones.contains(&coordenadas_ataque) {
                barcos_golpeados = true;
                barco.posiciones.retain(|&pos| pos != coordenadas_ataque); 

                if barco.posiciones.is_empty() {
                    barco.estado = EstadoBarco::Hundido;
                    println!("El barco ha sido hundido");
                    println!("Ganaste 15 puntos");
                    puntos += 15;
                    barcos_hundidos.push(coordenadas_ataque);

                } else {
                    if barco.estado == EstadoBarco::Sano {
                        barco.estado = EstadoBarco::Golpeado;
                        println!("Le pegaste a un barco");
                        println!("Ganaste 5 puntos");
                        puntos += 5;
                    } else if barco.estado == EstadoBarco::Golpeado {
                        println!("Ganaste 5 puntos");
                        puntos += 5;
                    }
                }
            }
        }

        self.barcos.retain(|barco| barco.estado != EstadoBarco::Hundido);

        for coordenadas in barcos_hundidos {
            self.mapa.marcar_hundido(coordenadas);
        }

        if !barcos_golpeados {
            println!("No le pegaste a nada, burro irrecuperable.");
        }
        puntos
    }
    fn enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }
    
}