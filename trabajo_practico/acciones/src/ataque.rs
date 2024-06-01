use serde::{Deserialize, Serialize};


/// Estructura de datos para los ataques
#[derive(Debug, Serialize, Deserialize)]
pub struct Ataque {
    pub jugador_id: usize,
    pub id_barco: usize,
    pub cordenadas_ataque: (i32, i32),
}
