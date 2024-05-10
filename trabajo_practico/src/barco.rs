pub struct Barco {
    pub nombre: String,
    pub tamaño: usize,
    pub posición: (usize, usize), 
}

impl Barco {
    pub fn new(nombre: String, tamaño: usize, posición: (usize, usize)) -> Barco {
        Barco {
            nombre,
            tamaño,
            posición,
        }
    }
}