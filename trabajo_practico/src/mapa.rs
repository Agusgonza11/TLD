use crate::{acciones::Accion, flota::Flota};
use ndarray::{Array2, s};
use rand::Rng;


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

    fn set_random(&mut self, ch: char) {
        let mut rng = rand::thread_rng();
        let (nrows, ncols) = (self.tablero.nrows(), self.tablero.ncols());
        let mut fil = 0;
        let mut col = 0;
        loop {
            let fil = rng.gen_range(0..nrows);
            let col = rng.gen_range(0..ncols);
            if self.tablero[[fil, col]] == '.' {
                break;
            }
        }
    }

    pub fn obtener_posicion_libre(&mut self, id: String) -> (usize, usize) {
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
        return (fil, col)
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

    pub fn modificar(&mut self, accion: Accion) {
        
    }


}