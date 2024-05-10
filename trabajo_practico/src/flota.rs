use crate::barco::Barco;

pub struct Flota {
    pub barcos: Vec<Barco>,
}

impl Flota {
    pub fn new() -> Flota {
        Flota { barcos: Vec::new() }
    }

    pub fn agregar_barco(&mut self, nombre: String, tamaño: usize, posición: (usize, usize)) {
        let nuevo_barco = Barco::new(nombre, tamaño, posición);
        self.barcos.push(nuevo_barco);
    }
}