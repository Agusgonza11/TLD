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
    pub ha_perdido: bool,
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

        let tamaños_barcos: Vec<usize> = vec![1];
        for (id_actual, &tamaño) in tamaños_barcos.iter().enumerate() {
            let vec_posiciones = mapa.obtener_posiciones_libres_contiguas(id.to_string(), tamaño);
            barcos.push(Barco::new(id_actual, tamaño, vec_posiciones));
        }

        Jugador {
            id,
            nombre_usuario: nombre,
            barcos,
            puntos: 0,
            monedas: 500,
            mapa: mapa.clone(),
            ha_perdido: false,
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
        if self.barcos.is_empty() {
            return;
        }
        let _ = self
            .mapa
            .enviar_tablero(self.id.to_string(), server, &self.barcos, self.monedas);
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
        let id_barco = self.barcos.len();
        self.barcos
            .push(Barco::new(id_barco, tamanio_barco, vec_posiciones));
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

        if self.mapa.actualizar_posicion_barco(
            &mut self.barcos[barco],
            coordenadas_destino.clone(),
            self.id,
        ) {
            self.barcos[barco].actualizar_posicion(coordenadas_destino);
        }
    }
    /// Función que permite al jugador obtener un barco
    ///
    /// # Args
    ///
    /// `barco_seleccionado` - Indice del barco seleccionado
    ///
    /// # Returns
    ///
    /// `Barco` - Barco seleccionado
    pub fn obtener_barco(&self, barco_seleccionado: usize) -> Barco {
        self.barcos[barco_seleccionado].clone()
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

    pub fn procesar_ataque(
        &mut self,
        coordenadas_ataque: (i32, i32),
        server: &Server,
    ) -> (usize, usize) {
        let mut puntos = 0;
        let mut monedas = 0;
        let mut barcos_hundidos = Vec::new();
        for barco in &mut self.barcos {
            if barco.posiciones.contains(&coordenadas_ataque) {
                barco.posiciones.retain(|&pos| pos != coordenadas_ataque);

                if barco.posiciones.is_empty() {
                    barco.estado = EstadoBarco::Hundido;
                    puntos += 15;
                    monedas += 100;

                    let conexion = server
                        .conexiones_jugadores
                        .get(&self.id)
                        .unwrap()
                        .lock()
                        .unwrap();
                    let mensaje_serializado =
                        serde_json::to_string(&Mensaje::BarcoHundido).unwrap();
                    let _ =
                        Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec());
                    barcos_hundidos.push(coordenadas_ataque);
                } else if barco.estado == EstadoBarco::Sano {
                    barco.estado = EstadoBarco::Golpeado;
                    let conexion = server
                        .conexiones_jugadores
                        .get(&self.id)
                        .unwrap()
                        .lock()
                        .unwrap();
                    let mensaje_serializado =
                        serde_json::to_string(&Mensaje::BarcoGolpead(coordenadas_ataque));
                    let _ = Self::enviar_mensaje(
                        &conexion,
                        mensaje_serializado.unwrap().as_bytes().to_vec(),
                    );
                    puntos += 5;
                    monedas += 50;
                } else if barco.estado == EstadoBarco::Golpeado {
                    let conexion = server
                        .conexiones_jugadores
                        .get(&self.id)
                        .unwrap()
                        .lock()
                        .unwrap();
                    let mensaje_serializado =
                        serde_json::to_string(&Mensaje::BarcoGolpead(coordenadas_ataque));
                    let _ = Self::enviar_mensaje(
                        &conexion,
                        mensaje_serializado.unwrap().as_bytes().to_vec(),
                    );
                    puntos += 5;
                    monedas += 50;
                }
            }
        }

        self.barcos
            .retain(|barco| barco.estado != EstadoBarco::Hundido);

        for coordenadas in barcos_hundidos {
            self.mapa.marcar_hundido(coordenadas);
        }

        (puntos, monedas)
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

    pub fn esta_vivo(&self) -> bool {
        !self.barcos.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapa::Mapa;

    #[test]
    fn test_new_jugador() {
        let jugador = Jugador::new(1, "Jugador 1".to_string(), &mut Mapa::new());
        assert_eq!(jugador.id, 1);
        assert_eq!(jugador.nombre_usuario, "Jugador 1");
        assert_eq!(jugador.barcos.len(), 1);
        assert_eq!(jugador.puntos, 0);
        assert_eq!(jugador.monedas, 500);
    }

    #[test]
    fn test_agregar_barco() {
        let mut jugador = Jugador::new(1, "Jugador 1".to_string(), &mut Mapa::new());
        jugador.agregar_barco(2);
        assert_eq!(jugador.barcos.len(), 2);
    }

    #[test]
    fn test_obtener_barco() {
        let mut jugador = Jugador::new(1, "Jugador 1".to_string(), &mut Mapa::new());
        jugador.agregar_barco(2);
        let barco = jugador.obtener_barco(1);
        assert_eq!(barco.tamaño, 2);
    }

    #[test]
    fn test_actualizar_posicion_barco() {
        let mut jugador = Jugador::new(1, "Jugador 1".to_string(), &mut Mapa::new());
        jugador.agregar_barco(2);
        jugador.actualizar_posicion_barco(vec![(0, 0), (0, 1)], 0);
        assert_eq!(jugador.barcos[0].posiciones, vec![(0, 0)]);
    }
}
