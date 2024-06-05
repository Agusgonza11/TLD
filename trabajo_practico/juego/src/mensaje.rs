use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Mensaje {
    PreguntaComienzo,
    RealiceAccion,
    Esperando,
    Puntos(String),
    Tablero(Vec<Vec<char>>, Vec<char>),
}