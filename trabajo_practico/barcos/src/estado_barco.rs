#[derive(Debug, Clone,PartialEq)]
/// Enumeración que representa el estado de un barco
pub enum EstadoBarco {
    Sano,
    Golpeado,
    Hundido,
}

impl EstadoBarco {
    pub fn to_bytes(&self) -> &[u8] {
        match self {
            EstadoBarco::Sano => &[0],
            EstadoBarco::Golpeado => &[1],
            EstadoBarco::Hundido => &[2],
        }
    }
}