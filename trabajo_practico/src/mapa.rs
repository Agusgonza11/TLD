use crate::flota::Flota;

pub struct Mapa {
    pub tablero: Vec<Vec<char>>,
    pub flotas: Vec<Flota>, 
}

impl Mapa {
    pub fn new() -> Mapa {
        let tablero = vec![vec!['.'; 40]; 20]; 
        let flotas = Vec::new(); 
        Mapa { tablero, flotas }
    }


    pub fn imprimir_tablero(&self) {
        for row in &self.tablero {
            println!("{}", row.iter().collect::<String>());
        }
    }

}