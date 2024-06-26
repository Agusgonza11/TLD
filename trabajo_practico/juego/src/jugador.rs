use acciones::accion::Accion;
use barcos::{barco::Barco, estado_barco::EstadoBarco};
use libreria::custom_error::CustomError;

use crate::{mapa::Mapa, mensaje::Mensaje, server::Server};
use std::{io::Write, net::TcpStream, vec};

#[derive(Clone)]
pub struct Jugador {
    pub id: usize,
    pub nombre_usuario: String,
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
    pub fn new(id: usize, nombre: String, mapa: &mut Mapa) -> Jugador {
        let mut barcos = Vec::new();
        let mut id_actual = 0;

        let tamaños_barcos: Vec<usize> = vec![1];
        for tamaño in tamaños_barcos {
            let vec_posiciones = mapa.obtener_posiciones_libres_contiguas(id.to_string(), tamaño);
            let id_barco = id_actual;
            id_actual += 1;
            barcos.push(Barco::new(id_barco, tamaño, vec_posiciones));
        }

        Jugador {
            id,
            nombre_usuario: nombre,
            barcos,
            puntos: 0,
            monedas: 500,
            mapa: mapa.clone(),
        }
    }
    /// Función que envía las instrucciones al jugador
    /// 
    /// # Args
    /// 
    /// `server` - Servidor en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// `()` - No retorna nada
    pub fn enviar_instrucciones(&self, server: &Server) {
        if let Some(conexion) = server.conexiones_jugadores.get(&self.id) {
            let conexion = conexion
                .lock()
                .map_err(|_| CustomError::ErrorAceptandoConexion)
                .unwrap();
            let mensaje_serializado = serde_json::to_string(&Mensaje::Puntos(self.puntos)).unwrap();
            let _ = Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec());
        }
    }
    /// Función que maneja el turno del jugador
    /// 
    /// # Args
    /// 
    /// `server` - Servidor en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// 
    pub fn manejar_turno(&mut self, server: &Server) {
        let _ = self
            .mapa
            .enviar_tablero(self.id.to_string(), server, &self.barcos, self.monedas.clone());
    }
    /// Función que permite al jugador agregar un barco al tablero
    /// 
    /// # Args
    /// 
    /// `tamanio_barco` - Tamaño del barco a agregar
    /// 
    /// # Returns
    /// 
    /// `()` - No retorna nada
    pub fn agregar_barco(&mut self, tamanio_barco: usize) {
        let vec_posiciones = self
            .mapa
            .obtener_posiciones_libres_contiguas(self.id.to_string(), tamanio_barco);
        let id_barco = self.barcos.len() ;
        self.barcos.push(Barco::new(id_barco, tamanio_barco, vec_posiciones));
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

    pub fn actualizar_posicion_barco(
        &mut self,
        coordenadas_contiguas: Vec<(i32, i32)>,
        barco: usize,
    ) {
        let mut coordenadas_destino = vec![];
        for (i, &coordenada) in coordenadas_contiguas.iter().enumerate() {
            coordenadas_destino.push(coordenada);
            if i == self.barcos[barco].tamaño - 1 {
                break;
            }
        }

        if self.mapa.actualizar_posicion_barco(&mut self.barcos[barco], coordenadas_destino.clone(), self.id) {
            self.barcos[barco].actualizar_posicion(coordenadas_destino);
        }
    }
    ///
    pub fn obtener_barco(&self, barco_seleccionado: usize) -> Result<Barco,CustomError> {
        if barco_seleccionado >= self.barcos.len() {
            return Err(CustomError::ErrorBarcoInexistente);
        }
        Ok(self.barcos[barco_seleccionado].clone())
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

    pub fn procesar_ataque(&mut self, coordenadas_ataque: (i32, i32)) -> (usize,usize) {
        let mut puntos = 0;
        let mut monedas = 0;
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
                    monedas += 100;
                    barcos_hundidos.push(coordenadas_ataque);
                } else {
                    if barco.estado == EstadoBarco::Sano {
                        barco.estado = EstadoBarco::Golpeado;
                        println!("Le pegaste a un barco");
                        println!("Ganaste 5 puntos");
                        puntos += 5;
                        monedas += 50;
                    } else if barco.estado == EstadoBarco::Golpeado {
                        println!("Ganaste 5 puntos");
                        puntos += 5;
                        monedas += 50;
                    }
                }
            }
        }

        self.barcos
            .retain(|barco| barco.estado != EstadoBarco::Hundido);

        for coordenadas in barcos_hundidos {
            self.mapa.marcar_hundido(coordenadas);
        }

        if !barcos_golpeados {
            //si no le pego a ningun braco se envia el mensaje al jugador 

            
            println!("No le pegaste a nada, burro irrecuperable.");
        }
        (puntos,monedas)
    }
    /// Función que envía un mensaje al servidor
    /// 
    /// # Args
    /// 
    /// `stream` - Stream por el cual se enviará el mensaje
    /// 
    /// `msg` - Mensaje a enviar
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la operación
    /// 
    /// # Errors
    fn enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }
}
