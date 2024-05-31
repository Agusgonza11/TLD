use barcos::flota::Flota;
use ndarray::Array2;
use rand::Rng;


#[derive(Clone)]

/// Estructura que representa el mapa del juego
pub struct Mapa {
    pub tablero: Array2<char>,
    pub flotas: Vec<Flota>,
}

impl Mapa {
    /// Función que crea un nuevo mapa
    /// 
    /// # Returns
    /// 
    /// `Mapa` - Mapa creado
    pub fn new() -> Mapa {
        let tablero = Array2::from_elem((10, 10), '.');
        let flotas = Vec::new();
        Mapa { tablero, flotas }
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
    pub fn actualizar_posicion_barco(&mut self, coordenadas_origen: &Vec<(i32, i32)>, coordenadas_destino: &Vec<(i32, i32)>, id: usize) {
        for &(x_origen, y_origen) in coordenadas_origen.iter() {
            if x_origen >= 0 && x_origen < self.tablero.ncols() as i32 && y_origen >= 0 && y_origen < self.tablero.nrows() as i32 {
                self.tablero[[y_origen as usize, x_origen as usize]] = '.';
            } else {
                println!("Coordenada origen fuera de limites: ({}, {})", x_origen, y_origen); 
            }
        }
    
        for &(x_destino, y_destino) in coordenadas_destino.iter() {
            println!("Actualizando posición destino: ({}, {})", x_destino, y_destino); // Depuración
            if x_destino >= 0 && x_destino < self.tablero.ncols() as i32 && y_destino >= 0 && y_destino < self.tablero.nrows() as i32 {
                self.tablero[[y_destino as usize, x_destino as usize]] = id.to_string().chars().next().unwrap(); 
            } else {
                println!("Coordenada destino fuera de limites: ({}, {})", x_destino, y_destino); 
            }
        }
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
    pub fn obtener_coordenadas_contiguas(&self, coordenada_destino: (i32, i32), tamano_barco: usize) -> Vec<(i32, i32)> {
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
    pub fn obtener_posiciones_libres_contiguas(&mut self, id: String, tamaño: usize) -> Vec<(i32, i32)> {
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
    
}