use crate::{custom_error::CustomError, mapa::Mapa };

pub struct Juego{
    pub mapa: Mapa,
    pub jugadores: Vec<String>,

}

impl Juego{
    pub fn new() -> Juego {
        let mapa = Mapa::new();
        let jugadores = Vec::new();
        Juego { mapa, jugadores}
    }
    pub fn iniciar_juego(&mut self) -> Result<(),CustomError>{
        Ok(())
    }
    
    pub fn agregar_jugador(&mut self, nombre: String) {
        self.jugadores.push(nombre);
        
    }

}