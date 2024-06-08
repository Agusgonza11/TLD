pub struct Serializador;
impl Serializador {
    pub fn serializar_nombre_de_usuario(&self, nombre: &str) -> Vec<u8> {
        let nombre_bytes = nombre.as_bytes();
        let nombre_longitud = nombre_bytes.len() as u32;

        let mut buffer = Vec::new();

        buffer.extend(&nombre_longitud.to_be_bytes());

        buffer.extend(nombre_bytes);

        buffer
    }
}
