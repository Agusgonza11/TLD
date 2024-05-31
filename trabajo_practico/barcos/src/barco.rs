use crate::estado_barco::EstadoBarco;

#[derive(Debug, Clone)]
/// Estructura que representa un barco
pub struct Barco {
    pub id: usize,
    pub tamaño: usize,
    pub posiciones: Vec<(i32, i32)>,
    pub estado: EstadoBarco,
}

impl Barco {
    /// Función que crea un nuevo barco
    pub fn new(id: usize, tamaño: usize, posiciones: Vec<(i32, i32)>)-> Barco {
        
        Barco {
            id,
            tamaño,
            posiciones, 
            estado: EstadoBarco::Sano,
        }
    }
}
