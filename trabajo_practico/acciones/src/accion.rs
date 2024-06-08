use crate::{ataque::Ataque, movimiento::Movimiento};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Enumeración que representa las acciones que puede realizar un jugador
pub enum Accion {
    Moverse(Movimiento),
    Atacar(Ataque),
    Tienda(usize),
    Saltar,
}
