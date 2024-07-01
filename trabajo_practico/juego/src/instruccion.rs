use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum Instruccion {
    Ataque(usize, (i32, i32)),
    Movimiento(usize, (i32, i32)),
    Compra(usize),
    Saltar,
    Ranking,
}
