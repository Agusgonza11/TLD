use crate::{custom_error::CustomError, jugador::Jugador, mapa::Mapa };

pub struct Juego{
    pub mapa: Mapa,
    pub jugadores: Vec<Jugador>,
    pub turno: usize,
}

impl Juego{
    pub fn new(numero_jugadores: usize) -> Juego {
        let mut mapa = Mapa::new();
        let mut jugadores = Vec::new();
        for t in 1..=numero_jugadores{
            jugadores.push(Jugador::new(t, &mut mapa));
        }
        let turno = 0;
        Juego { mapa, jugadores, turno }
    }

    pub fn iniciar_juego(&mut self) -> Result<(),CustomError>{
        //self.mapa.imprimir_tablero();

        while !self.finalizo() {
            let jugador_actual = &mut self.jugadores[self.turno];
            println!("Turno del jugador {}", jugador_actual.id);
            Self::imprimir_acciones();

            self.mapa.modificar(jugador_actual.turno(&self.mapa));


            self.turno = (self.turno + 1) % self.jugadores.len();
        }
        Ok(())
    }

    fn finalizo(&self) -> bool {
        self.jugadores.len() <= 1
    }

    fn imprimir_acciones() {
        println!("Realice una accion: ");
        println!("Puede moverse: (m)");
        println!("Puede atacar: (a)");
        println!("Puede abrir la tienda: (t)");
    }
    
    pub fn agregar_jugador(&mut self, jugador: Jugador) {
        self.jugadores.push(jugador);
    }

}