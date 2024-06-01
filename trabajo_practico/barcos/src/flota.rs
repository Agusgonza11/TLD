use crate::{barco::Barco, estado_barco::EstadoBarco};

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

    pub fn from_bytes(bytes: &[u8]) -> (Flota, usize) {
        let mut offset = 0;

        // Lee el número de barcos de los primeros 4 bytes
        let num_barcos = u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]) as usize;
        offset += 4;

        // Crea un vector para almacenar los barcos
        let mut barcos = Vec::with_capacity(num_barcos);

        // Itera sobre cada barco en los bytes restantes
        for _ in 0..num_barcos {
            // Lee el ID del barco de los siguientes 4 bytes
            let id = u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]) as usize;
            offset += 4;

            // Lee el tamaño del barco de los siguientes 4 bytes
            let tamaño = u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]) as usize;
            offset += 4;

            // Lee las posiciones del barco de los siguientes 8 * 2 * 10 bytes
            let mut posiciones = Vec::new();
            for _ in 0..tamaño {
                let x = i32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
                let y = i32::from_be_bytes([bytes[offset + 4], bytes[offset + 5], bytes[offset + 6], bytes[offset + 7]]);
                posiciones.push((x, y));
                offset += 8;
            }

            // Lee el estado del barco del siguiente byte
            let estado = match bytes[offset] {
                0 => EstadoBarco::Sano,
                1 => EstadoBarco::Golpeado,
                2 => EstadoBarco::Hundido,
                _ => panic!("Valor de estado no válido"),
            };
            offset += 1;

            // Crea el barco y agrégalo al vector
            let barco = Barco { id, tamaño, posiciones, estado };
            barcos.push(barco);
        }

        // Crea la flota y devuelve el resultado junto con el número de bytes leídos
        (Flota { barcos }, offset)
    }
}
