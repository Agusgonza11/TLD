use crate::barco::Barco;
#[derive(Debug, Clone)]
pub struct Flota {
    pub barcos: Vec<Barco>,
}
impl Default for Flota {
    fn default() -> Self {
        Self::new()
    }
}
impl Flota {
    pub fn new() -> Flota {
        Flota { barcos: Vec::new() }
    }

    pub fn agregar_barco(&mut self, id: usize, tamaño: usize, posiciones: Vec<(i32, i32)>) {
        let nuevo_barco = Barco::new(id, tamaño, posiciones);
        self.barcos.push(nuevo_barco);
    }
}
