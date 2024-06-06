/// Estructura de datos para los ataques
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ataque {
    pub jugador_id: usize,
    pub id_barco: usize,
    pub cordenadas_ataque: (i32, i32),
}
