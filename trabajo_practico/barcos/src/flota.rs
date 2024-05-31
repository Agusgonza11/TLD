use crate::barco::Barco;
#[derive(Debug, Clone)]
pub struct Flota {
    pub barcos: Vec<Barco>,
}

impl Flota {
    pub fn new() -> Flota {
        Flota { barcos: Vec::new() }
    }

    pub fn agregar_barco(&mut self, id: usize, tamaño: usize, posición: (i32, i32)) {
        let nuevo_barco = Barco::new(id, tamaño, posición);
        self.barcos.push(nuevo_barco);
    }
}
