use crate::{acciones::Accion, flota::Flota};
use ndarray::{Array2, s};

pub struct Mapa {
    pub tablero: Array2<char>,
    pub flotas: Vec<Flota>, 
}

impl Mapa {
    pub fn new() -> Mapa {
        let tablero = Array2::from_elem((5, 5), '.'); 
        let flotas = Vec::new(); 
        Mapa { tablero, flotas }
    }

    fn set(&mut self, row: usize, col: usize, ch: char) {
        if row < self.tablero.nrows() && col < self.tablero.ncols() {
            self.tablero[[row, col]] = ch;
        } else {
            println!("Index out of bounds!");
        }
    }

    pub fn imprimir_tablero(&self) {
        for row in self.tablero.rows() {
            for &cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn modificar(&mut self, accion: Accion) {
        
    }


}