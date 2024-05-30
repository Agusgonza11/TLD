use crate::estado_barco::EstadoBarco;

#[derive(Debug, Clone)]
pub struct Barco {
    pub id: usize,
    pub tamaño: usize,
    pub posicion: (i32, i32),
    pub estado: EstadoBarco,
}
impl Barco {
    pub fn new(id: usize, tamaño: usize, posicion: (i32, i32)) -> Barco {
        Barco {
            id,
            tamaño,
            posicion,
            estado: EstadoBarco::Sano,
        }
    }
}
