use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum Instruccion {
    Ataque(usize, (i32, i32)),
    Movimiento(usize, (i32, i32)),
    Compra(usize),
    Saltar,
    Ranking,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Mensaje {
    PreguntaComienzo,
    RealiceAccion,
    Esperando,
    Puntos(usize),
    Tablero(Vec<Vec<char>>, Vec<(usize, Vec<(i32, i32)>)>, usize),
    Accion(Instruccion),
    AbrirTienda(usize),
    RepetirAccion(String, Vec<(usize, Vec<(i32, i32)>)>, usize),
    MensajeBarcoGolpeado,
    MensajeBarcoHundido,
    MensajeInfoAaque(usize, usize),
    EventoSorpresa,
    EventoSorpresaResultado(bool),
    Registro,
    Ranking(Vec<(String, usize)>),
    FinPartida(String,usize),
}
