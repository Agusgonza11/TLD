use crate::{custom_error::CustomError, mapa::Mapa, jugador::Jugador };

pub struct Juego{
    pub mapa: Mapa,
    pub jugadores: Vec<Jugador>,
    pub turno: i32,
}

impl Juego{
    pub fn new(numero_jugadores: i32) -> Juego {
        let mapa = Mapa::new();
        let mut jugadores = Vec::new();
        for t in 1..numero_jugadores{
            jugadores.push(Jugador::new(t));
        }
        let turno = 0;
        Juego { mapa, jugadores, turno }
    }

    pub fn iniciar_juego(&mut self) -> Result<(),CustomError>{
        self.mapa.imprimir_tablero();
        Ok(())
    }
    
    pub fn agregar_jugador(&mut self, jugador: Jugador) {
        self.jugadores.push(jugador);
        
    }

}