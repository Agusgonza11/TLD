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
    ///
    /// # Args
    ///
    /// `id` - Identificador del barco
    ///
    /// `tamaño` - Tamaño del barco
    ///
    /// `posiciones` - Posiciones del barco
    ///
    /// # Returns
    ///
    /// `Barco` - Barco creado
    pub fn new(id: usize, tamaño: usize, posiciones: Vec<(i32, i32)>) -> Barco {
        Barco {
            id,
            tamaño,
            posiciones,
            estado: EstadoBarco::Sano,
        }
    }
    /// Función que obtiene los datos del barco
    ///
    /// # Returns
    ///
    /// `(usize, Vec<(i32, i32)>)` - Tupla con el id y las posiciones del barco
    pub fn obtener_datos(&self) -> (usize, Vec<(i32, i32)>) {
        (self.id, self.posiciones.clone())
    }
    /// Función que actualiza la posición del barco
    ///
    /// # Args
    ///
    /// `nueva_posicion` - Nueva posición del barco
    ///
    /// # Returns
    ///
    /// `()` - No retorna nada
    pub fn actualizar_posicion(&mut self, nueva_posicion: Vec<(i32, i32)>) {
        self.posiciones = nueva_posicion;
    }
}
