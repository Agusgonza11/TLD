use crate::{ataque::Ataque, movimiento::Movimiento};

pub enum Accion {
    Moverse(Movimiento),
    Atacar(Ataque),
    Tienda(usize),
    Saltar,
}
