use crate::{ataque::Ataque, movimiento::Movimiento};
use serde::{Deserialize, Serialize};

/// Enumeración que representa las acciones que puede realizar un jugador
#[derive(Debug, Serialize, Deserialize)]
pub enum Accion {
    Moverse(Movimiento),
    Atacar(Ataque),
    Tienda(usize),
    Saltar,
}
