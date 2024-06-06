use crate::{ataque::Ataque, movimiento::Movimiento};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
/// Enumeración que representa las acciones que puede realizar un jugador
pub enum Accion {
    Moverse(Movimiento),
    Atacar(Ataque),
    Tienda(usize),
    Saltar,
}
