use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum Instruccion {
    Ataque(usize, (i32, i32)),
    Movimiento(usize, (i32, i32)),
    Tienda,
    Saltar,
    Ranking,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Mensaje {
    PreguntaComienzo,
    RealiceAccion,
    Esperando,
    Puntos(usize),
    Tablero(Vec<Vec<char>>, Vec<(usize, Vec<(i32, i32)>)>),
    Accion(Instruccion),
    RepetirAccion(String, Vec<(usize, Vec<(i32, i32)>)>),
    EventoSorpresa,
    EventoSorpresaResultado(bool),
    Registro,
    Ranking(Vec<(String, usize)>),
}
