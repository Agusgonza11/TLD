use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Mensaje {
    PreguntaComienzo,
    RealiceAccion,
    Esperando,
    Puntos(usize),
    Tablero(Vec<Vec<char>>, Vec<Vec<char>>),
}