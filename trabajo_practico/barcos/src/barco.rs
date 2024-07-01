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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let barco = Barco::new(1, 2, vec![(1, 1), (1, 2)]);
        assert_eq!(barco.id, 1);
        assert_eq!(barco.tamaño, 2);
        assert_eq!(barco.posiciones, vec![(1, 1), (1, 2)]);
        assert_eq!(barco.estado, EstadoBarco::Sano);
    }

    #[test]
    fn test_obtener_datos() {
        let barco = Barco::new(1, 2, vec![(1, 1), (1, 2)]);
        assert_eq!(barco.obtener_datos(), (1, vec![(1, 1), (1, 2)]));
    }

    #[test]
    fn test_actualizar_posicion() {
        let mut barco = Barco::new(1, 2, vec![(1, 1), (1, 2)]);
        barco.actualizar_posicion(vec![(2, 2), (2, 3)]);
        assert_eq!(barco.posiciones, vec![(2, 2), (2, 3)]);
    }
}