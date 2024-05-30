use crate:: flota::Flota;
use ndarray::Array2;
use rand::Rng;


#[derive(Clone)]
pub struct Mapa {
    pub tablero: Array2<char>,
    pub flotas: Vec<Flota>,
}

impl Mapa {
    pub fn new() -> Mapa {
        let tablero = Array2::from_elem((10, 10), '.');
        let flotas = Vec::new();
        Mapa { tablero, flotas }
    }

    fn set(&mut self, row: usize, col: usize, ch: char) {
        if row < self.tablero.nrows() && col < self.tablero.ncols() {
            self.tablero[[row, col]] = ch;
        } else {
            println!("index error");
        }
    }

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

    pub fn actualizar_posicion_barco(&mut self, coordenadas_origen: (i32, i32), coordenadas_destino: (i32, i32), id: char) {
        let (x_origen, y_origen) = coordenadas_origen;
        let (x_destino, y_destino) = coordenadas_destino;
        
        if x_origen >= 0 && x_origen < self.tablero.ncols() as i32 && y_origen >= 0 && y_origen < self.tablero.nrows() as i32 {
            self.tablero[[y_origen as usize, x_origen as usize]] = '.';
        }
        
        if x_destino >= 0 && x_destino < self.tablero.ncols() as i32 && y_destino >= 0 && y_destino < self.tablero.nrows() as i32 {
            self.tablero[[y_destino as usize, x_destino as usize]] = id;
        }
    }

    pub fn marcar_hundido(&mut self, coordenadas: (i32, i32)) {
        let (x, y) = coordenadas;
        if x >= 0 && x < self.tablero.ncols() as i32 && y >= 0 && y < self.tablero.nrows() as i32 {
            self.tablero[[y as usize, x as usize]] = 'X';
        }
    }
}