use libreria::custom_error::CustomError;

pub struct Deserializador;

impl Deserializador {
    pub fn deserializar_nombre_de_usuario(buffer: &[u8]) -> Result<String, CustomError> {
        if buffer.len() < 4 {
            return Err(CustomError::LongitudNombreInvalida);
        }

        let nombre_longitud =
            u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;

        if buffer.len() < nombre_longitud + 4 {
            return Err(CustomError::LongitudNombreInvalida);
        }

        let nombre_bytes = &buffer[4..nombre_longitud + 4];
        let nombre = String::from_utf8_lossy(nombre_bytes).to_string();

        Ok(nombre)
    }
}
