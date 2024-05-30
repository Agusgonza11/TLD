use crate::{accion::Accion, custom_error::CustomError, jugador::Jugador, mapa:: Mapa, movimiento::Movimiento};

#[derive(Clone)]
pub struct Juego {
    pub mapa: Mapa,
    pub jugadores: Vec<Jugador>,
    pub turno: usize,
}

impl Juego {
    pub fn new(numero_jugadores: usize) -> Juego {
        let mut mapa = Mapa::new();
        let mut jugadores = Vec::new();
        for _ in 0..numero_jugadores {
            jugadores.push(Jugador::new(jugadores.len(), &mut mapa));
        }
        let turno = 0;
        Juego {
            mapa,
            jugadores,
            turno,
        }
    }

    pub fn iniciar_juego(&mut self) -> Result<(), CustomError> {
        while !self.finalizo() {
            let jugador_actual = &mut self.jugadores[self.turno];
            println!("Turno del jugador {}", jugador_actual.id);
            Self::imprimir_acciones();

            let accion = jugador_actual.turno(&self.mapa);
            match accion {
                Accion::Moverse(movimiento) => {
                    Self::procesar_movimiento(movimiento, &mut self.jugadores);
                },
                Accion::Atacar(ataque) => {
                    Self::procesar_ataque(ataque.cordenadas_ataque, ataque.jugador_id, &mut self.jugadores,&mut self.mapa);
                },
                Accion::Saltar => {
                    println!("Jugador {} salta su turno.", jugador_actual.id);
                },
                _ => {},
            }

            self.turno = (self.turno + 1) % self.jugadores.len();
        }
        Ok(())
    }

    fn finalizo(&self) -> bool {
        let jugadores_con_barcos = self.jugadores.iter().filter(|j| !j.barcos.is_empty()).count();
        if jugadores_con_barcos <= 1 {
            if let Some(jugador) = self.jugadores.iter().find(|j| !j.barcos.is_empty()) {
                println!("El jugador {} ha ganado", jugador.id);
            } else {
                println!("No hay ganadores.");
            }
            return true;
        }
        false
    }

    fn imprimir_acciones() {
        println!("Realice una accion: ");
        println!("Puede moverse: (m)");
        println!("Puede atacar: (a)");
        println!("Puede abrir la tienda: (t)");
        println!("Puede saltar turno: (s)");
    }

    pub fn agregar_jugador(&mut self) {
        let ultimo_id = self.jugadores.len();
        self.jugadores.push(Jugador::new(ultimo_id, &mut self.mapa));
    }

    fn procesar_movimiento(movimiento: Movimiento, jugadores: &mut Vec<Jugador>) {
        for jugador in jugadores.iter_mut() {
            if jugador.id == movimiento.jugador_id {
                for barco in &mut jugador.barcos {
                    if barco.id == movimiento.id_barco {
                        barco.posicion = movimiento.cordenadas_destino;
                        println!("Barco {} del jugador {} se ha movido a {:?}", barco.id, jugador.id, barco.posicion);
                    }
                }
                break;
            }
        }
    }

    fn procesar_ataque(coordenadas_ataque: (i32, i32), jugador_id: usize, jugadores: &mut Vec<Jugador>, mapa: &mut Mapa) {
        for jugador in jugadores.iter_mut() {
            if jugador.id != jugador_id {
                jugador.procesar_ataque(coordenadas_ataque, mapa);
            }
        }
    }
}