use std::{io::Write, net::TcpStream};

use barcos::barco::Barco;
use libreria::custom_error::CustomError;
use ndarray::Array2;
use rand::Rng;

use crate::{mensaje::Mensaje, server::Server};

#[derive(Clone)]

/// Estructura que representa el mapa del juego
pub struct Mapa {
    pub tablero: Array2<char>,
}
impl Default for Mapa {
    fn default() -> Self {
        Self::new()
    }
}

impl Mapa {
    /// Función que crea un nuevo mapa
    ///
    /// # Returns
    ///
    /// `Mapa` - Mapa creado
    pub fn new() -> Mapa {
        let tablero = Array2::from_elem((10, 10), '.');
        Mapa { tablero }
    }
    /// Función que establece un valor en una posición del tablero
    ///
    /// # Args
    ///
    /// `row` - Fila en la que se encuentra la posición
    ///
    /// `col` - Columna en la que se encuentra la posición
    ///
    /// `ch` - Caracter que se establecerá en la posición
    ///
    /// # Returns
    ///
    /// `()` - No retorna nada
    fn set(&mut self, row: usize, col: usize, ch: char) {
        if row < self.tablero.nrows() && col < self.tablero.ncols() {
            self.tablero[[row, col]] = ch;
        } else {
            println!("index error");
        }
    }
    /// Función que obtiene una posición libre en el tablero
    ///
    /// # Args
    ///
    /// `id` - Identificador del jugador
    ///
    /// # Returns
    ///
    /// `(i32, i32)` - Coordenadas de la posición libre
    pub fn obtener_posicion_libre(&mut self, id: String) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        let (nrows, ncols) = (self.tablero.nrows(), self.tablero.ncols());
        let mut fil;
        let mut col;
        let jugador: char = id.chars().next().unwrap();
        loop {
            fil = rng.gen_range(0..nrows);
            col = rng.gen_range(0..ncols);
            if self.tablero[[fil, col]] == '.' {
                self.set(fil, col, jugador);
                break;
            }
        }

        let fil_i32 = i32::try_from(fil).expect("Error");
        let col_i32 = i32::try_from(col).expect("Error");

        (fil_i32, col_i32)
    }
    /// Función que imprime el tablero
    ///
    /// # Args
    ///
    /// `id` - Identificador del jugador
    ///
    /// # Returns
    ///
    /// `()` - No retorna nada
    pub fn imprimir_tablero(&self, id: String) {
        let jugador: char = id.chars().next().unwrap();
        for row in self.tablero.rows() {
            for &cell in row.iter() {
                if cell != '.' && cell != jugador {
                    print!(".");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
    pub fn enviar_tablero(
        &self,
        id: String,
        server: &Server,
        barcos: &[Barco],
        monedas: usize,
    ) -> Result<(), CustomError> {
        let jugador: char = id
            .chars()
            .next()
            .ok_or(CustomError::ErrorAceptandoConexion)?;

        let mut tablero_ocultado = self.tablero.clone();
        for ((_, _), cell) in tablero_ocultado.indexed_iter_mut() {
            if *cell != '.' && *cell != jugador {
                *cell = '.';
            }
        }
        let tablero_vec: Vec<Vec<char>> = tablero_ocultado
            .outer_iter()
            .map(|row| row.to_vec())
            .collect();

        let barcos_serializados = self.serializar_barcos(barcos);

        if let Some(conexion) = server
            .conexiones_jugadores
            .get(&id.parse().unwrap_or_default())
        {
            let conexion = conexion
                .lock()
                .map_err(|_| CustomError::ErrorAceptandoConexion)?;
            let mensaje_serializado =
                serde_json::to_string(&Mensaje::Tablero(tablero_vec, barcos_serializados, monedas))
                    .unwrap();
            Self::enviar_mensaje(&conexion, mensaje_serializado.as_bytes().to_vec())?;
        }

        Ok(())
    }

    pub fn serializar_barcos(&self, barcos: &[Barco]) -> Vec<(usize, Vec<(i32, i32)>)> {
        let mut barcos_serializados = Vec::new();
        for barco in barcos.iter() {
            let barco = barco.obtener_datos();
            barcos_serializados.push(barco);
        }
        barcos_serializados
    }

    /// Funcion que actualiza la posición de un barco en el tablero
    ///
    /// # Args
    ///
    /// `coordenadas_origen` - Coordenadas de origen del barco
    ///
    /// `coordenadas_destino` - Coordenadas de destino del barco
    ///
    /// `id` - Identificador del barco
    ///
    /// # Returns
    ///
    /// `()` - No retorna nada
    pub fn actualizar_posicion_barco(
        &mut self,
        barco: &mut Barco,
        coordenadas_destino: Vec<(i32, i32)>,
        id: usize,
    ) -> bool {
        let mut modifico = false;
        let coordenadas_origen = barco.posiciones.clone();
        for &(x_origen, y_origen) in coordenadas_origen.iter() {
            if x_origen >= 0
                && x_origen < self.tablero.ncols() as i32
                && y_origen >= 0
                && y_origen < self.tablero.nrows() as i32
            {
                self.tablero[[y_origen as usize, x_origen as usize]] = '.';
            } else {
                println!(
                    "Coordenada origen fuera de limites: ({}, {})",
                    x_origen, y_origen
                );
            }
        }

        for &(x_destino, y_destino) in coordenadas_destino.iter() {
            if x_destino >= 0
                && x_destino < self.tablero.ncols() as i32
                && y_destino >= 0
                && y_destino < self.tablero.nrows() as i32
            {
                modifico = true;
                self.tablero[[y_destino as usize, x_destino as usize]] =
                    id.to_string().chars().next().unwrap();
            } else {
                println!(
                    "Coordenada destino fuera de limites: ({}, {})",
                    x_destino, y_destino
                );
            }
        }
        modifico
    }

    /// Función que marca una posición como hundida en el tablero
    ///
    /// # Args
    ///
    /// `coordenadas` - Coordenadas de la posición a marcar
    ///
    /// # Returns
    ///
    ///
    pub fn marcar_hundido(&mut self, coordenadas: (i32, i32)) {
        let (x, y) = coordenadas;
        if x >= 0 && x < self.tablero.ncols() as i32 && y >= 0 && y < self.tablero.nrows() as i32 {
            self.tablero[[y as usize, x as usize]] = 'X';
        }
    }

    /// Función que obtiene las coordenadas contiguas a una posicion dada
    ///
    /// # Args
    ///
    /// `coordenada_destino` - Coordenadas de la posición
    ///
    /// `tamaño_barco` - Tamaño del barco
    ///
    /// # Returns
    ///
    /// `Vec<(i32, i32)>` - Coordenadas contiguas
    pub fn obtener_coordenadas_contiguas(
        &self,
        coordenada_destino: (i32, i32),
        tamano_barco: usize,
    ) -> Vec<(i32, i32)> {
        let mut coordenadas_contiguas = Vec::new();
        let (x, y) = coordenada_destino;

        if self.es_coordenada_vacia(coordenada_destino) {
            coordenadas_contiguas.push((x, y));

            for i in 1..tamano_barco {
                let coordenada_horizontal = (x + i as i32, y);
                if self.es_coordenada_vacia(coordenada_horizontal) {
                    coordenadas_contiguas.push(coordenada_horizontal);
                } else {
                    coordenadas_contiguas.clear();
                    break;
                }
            }

            if coordenadas_contiguas.len() == tamano_barco {
                for i in 1..tamano_barco {
                    let coordenada_vertical = (x, y + i as i32);
                    if self.es_coordenada_vacia(coordenada_vertical) {
                        coordenadas_contiguas.push(coordenada_vertical);
                    } else {
                        coordenadas_contiguas.clear();
                        break;
                    }
                }
            }
        }

        coordenadas_contiguas
    }
    /// Función que obtiene posiciones libres contiguas en el tablero
    ///
    /// # Args
    ///
    /// `id` - Identificador del jugador
    ///
    /// `tamaño` - Tamaño del barco
    ///
    /// # Returns
    ///
    /// `Vec<(i32, i32)>` - Posiciones libres contiguas
    pub fn obtener_posiciones_libres_contiguas(
        &mut self,
        id: String,
        tamaño: usize,
    ) -> Vec<(i32, i32)> {
        let mut rng = rand::thread_rng();
        let (nrows, ncols) = (self.tablero.nrows(), self.tablero.ncols());
        let jugador: char = id.chars().next().unwrap();

        loop {
            let fil = rng.gen_range(0..nrows) as i32;
            let col = rng.gen_range(0..ncols) as i32;

            let mut posiciones = Vec::new();
            for i in 0..tamaño {
                let coord = (col + i as i32, fil);
                if self.es_coordenada_vacia(coord) {
                    posiciones.push(coord);
                } else {
                    break;
                }
            }

            if posiciones.len() == tamaño {
                for &(x, y) in &posiciones {
                    self.tablero[[y as usize, x as usize]] = jugador;
                }
                return posiciones;
            }
        }
    }

    /// Función que verifica si una coordenada está vacía
    ///
    /// # Args
    ///
    /// `coordenada` - Coordenada a verificar
    ///
    /// # Returns
    ///
    /// `bool` - Verdadero si la coordenada está vacía, falso en caso contrario
    pub fn es_coordenada_vacia(&self, coordenada: (i32, i32)) -> bool {
        let (x, y) = coordenada;
        if x >= 0 && y >= 0 && x < self.tablero.ncols() as i32 && y < self.tablero.nrows() as i32 {
            return self.tablero[[y as usize, x as usize]] == '.';
        }
        false
    }
    fn enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_mapa_new(){
        let mapa = Mapa::new();
        assert_eq!(mapa.tablero.nrows(), 10);
        assert_eq!(mapa.tablero.ncols(), 10);
    }

    #[test]
    fn test_mapa_set(){
        let mut mapa = Mapa::new();
        mapa.set(0, 0, 'a');
        assert_eq!(mapa.tablero[[0, 0]], 'a');
    }

    #[test]
    fn test_mapa_obtener_posicion_libre(){
        let mut mapa = Mapa::new();
        let (x, y) = mapa.obtener_posicion_libre("a".to_string());
        assert_eq!(mapa.tablero[[y as usize, x as usize]], '.');
    }

    #[test]
    fn test_mapa_imprimir_tablero(){
        let mut mapa = Mapa::new();
        mapa.set(0, 0, 'a');
        let  output = Vec::new();
        let _ = std::io::stdout().write_all(&output);
        mapa.imprimir_tablero("a".to_string());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "");
    }



}