use crate::instruccion::Instruccion;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Mensaje {
    PreguntaComienzo,
    RealiceAccion,
    Esperando,
    ComenzoJuego,
    Puntos(usize),
    Perdiste(usize),
    NotificacionEliminacion(String),
    Tablero(Vec<Vec<char>>, Vec<(usize, Vec<(i32, i32)>)>, usize),
    Accion(Instruccion, usize),
    AbrirTienda(usize),
    RepetirAccion(String, Vec<(usize, Vec<(i32, i32)>)>, usize),
    BarcoGolpead((i32, i32)),
    BarcoHundido,
    MensajeInfoAaque(usize, usize),
    EventoSorpresa,
    EventoSorpresaResultado(bool),
    Registro,
    Ranking(Vec<(String, usize)>),
    CompraExitosa(usize, usize),
    NotificacionCompra(String, usize),
    FinPartida(String, usize),
}
