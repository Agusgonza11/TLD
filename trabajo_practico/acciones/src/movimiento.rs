use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Movimiento {
    pub jugador_id: usize,
    pub id_barco: usize,
    pub coordenadas_origen: (i32, i32),
    pub cordenadas_destino: Vec<(i32, i32)>,
}
