pub struct Movimiento {
    pub jugador_id: usize,
    pub id_barco: usize,
    pub cordenadas_origen: (i32, i32),
    pub cordenadas_destino: (i32, i32),
}

pub struct Ataque {
    pub jugador_id: usize,
    pub id_barco: usize,
    pub cordenadas_ataque: (i32, i32),
}

pub enum Accion {
    Moverse(Movimiento),
    Atacar(Ataque),
    Saltar,
}