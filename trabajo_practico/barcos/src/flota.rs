use crate::barco::Barco;
#[derive(Debug, Clone)]
pub struct Flota {
    pub barcos: Vec<Barco>,
}

impl Flota {
    pub fn new() -> Flota {
        Flota { barcos: Vec::new() }
    }

    pub fn agregar_barco(&mut self, id: usize, tamaño: usize, posiciones: Vec<(i32, i32)>) {
        let nuevo_barco = Barco::new(id, tamaño, posiciones);
        self.barcos.push(nuevo_barco);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Calcular el tamaño total necesario para almacenar todos los datos
        let total_size = 4 + self.barcos.len() * (4 + 4 + 8 * 2 * 10 + 1); // 4 bytes para el tamaño de la flota, 4 bytes para id y tamaño, 8 bytes por posición (x, y), 1 byte para el estado del barco

        // Crear un vector de bytes con capacidad suficiente
        let mut bytes = Vec::with_capacity(total_size);

        // Convertir el número de barcos a bytes y agregar al vector
        bytes.extend_from_slice(&(self.barcos.len() as u32).to_be_bytes());

        // Iterar sobre cada barco en la flota
        for barco in &self.barcos {
            // Convertir los campos de Barco a bytes y agregar al vector
            bytes.extend_from_slice(&barco.id.to_be_bytes());
            bytes.extend_from_slice(&barco.tamaño.to_be_bytes());

            // Convertir las posiciones del barco a bytes y agregar al vector
            for &(x, y) in &barco.posiciones {
                bytes.extend_from_slice(&x.to_be_bytes());
                bytes.extend_from_slice(&y.to_be_bytes());
            }

            // Convertir el estado del barco a bytes y agregar al vector
            bytes.extend_from_slice(&barco.estado.to_bytes());
        }

        // Devolver un slice de bytes del vector
        bytes
    }
}
