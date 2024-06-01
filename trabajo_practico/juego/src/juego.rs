use acciones::{accion::Accion, movimiento::Movimiento};
use libreria::custom_error::CustomError;

use crate::{jugador::Jugador, mapa::Mapa};


pub struct Juego {
    pub mapa: Mapa,
    pub jugadores: Vec<Jugador>,
    pub turno: usize,
}

impl Juego {

    pub fn new() -> Juego {
        let mut mapa = Mapa::new();
        let jugadores = Vec::new();
        let turno = 0;
        Juego {
            mapa,
            jugadores,
            turno,
        }
    }


    /// Función que crea un nuevo juego
    /// 
    /// # Args
    /// 
    /// `numero_jugadores` - Número de jugadores que participarán en el juego
    /// 
    /// # Returns
    /// 
    /// `Juego` - Juego creado
    /// 

    /// Función que inicia el juego
    /// 
    /// # Returns
    /// 
    /// `Result<(), CustomError>` - Resultado de la ejecución
    /// 
    /// # Errors
    /// 
    /// `CustomError` - Error personalizado
    pub fn iniciar_juego(&mut self) -> Result<(), CustomError> {
        while !self.finalizo() {
            let jugador_actual = &mut self.jugadores[self.turno];
            println!("Turno del jugador {}", jugador_actual.id);
            Self::imprimir_acciones();

            let accion = jugador_actual.turno();
            match accion {
                Accion::Moverse(movimiento) => {
                    Self::procesar_movimiento(movimiento, &mut self.jugadores);
                }
                Accion::Atacar(ataque) => {
                    Self::procesar_ataque(ataque.cordenadas_ataque, ataque.jugador_id, &mut self.jugadores);
                }
                Accion::Saltar => {
                    println!("Jugador {} salta su turno.", jugador_actual.id);
                }
                _ => {
                    return Err(CustomError::AccionInvalida)
                }
            }

            self.turno = (self.turno + 1) % self.jugadores.len();
        }
        Ok(())
    }
    /// Función que verifica si el juego ha finalizado
    /// 
    /// # Returns
    /// 
    /// `bool` - Indica si el juego ha finalizado
    
    pub fn modificar(&mut self, accion: Accion) -> Result<(), CustomError> {
        match accion {
            Accion::Moverse(movimiento) => {
                Self::procesar_movimiento(movimiento, &mut self.jugadores);
            }
            Accion::Atacar(ataque) => {
                Self::procesar_ataque(ataque.cordenadas_ataque, ataque.jugador_id, &mut self.jugadores);
            }
            Accion::Saltar => {
                println!("Jugador salta su turno.");
            }
            _ => {
                return Err(CustomError::AccionInvalida)
            }
        }
        Ok(())
    }

    pub fn agregar_jugador(&mut self, id: usize) {
        self.jugadores.push(Jugador::new(id, &mut self.mapa));
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
    /// Función que imprime las acciones que puede realizar un jugador
    fn imprimir_acciones() {
        println!("Realice una accion: ");
        println!("Puede moverse: (m)");
        println!("Puede atacar: (a)");
        println!("Puede abrir la tienda: (t)");
        println!("Puede saltar turno: (s)");
    }
    /// Función que agrega un jugador al juego
    /// 
    /// # Args
    /// 
    /// `jugadores` - Vector de jugadores
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador agregado

    /// Función que procesa un movimiento en el mapa
    /// 
    /// # Args
    /// 
    /// `movimiento` - Movimiento a procesar
    /// 
    /// `jugadores` - Vector de jugadores
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador con el movimiento procesado
    fn procesar_movimiento(movimiento: Movimiento, jugadores: &mut Vec<Jugador>) {
        for jugador in jugadores.iter_mut() {
            if jugador.id == movimiento.jugador_id {
                for barco in &mut jugador.barcos {
                    if barco.id == movimiento.id_barco {
                        let nuevas_posiciones = movimiento.cordenadas_destino.clone();
                        let posicion_inicial = barco.posiciones.clone();
                        barco.posiciones = nuevas_posiciones.clone();
                        println!("Barco {} del jugador {} se ha movido a {:?}", barco.id, jugador.id, barco.posiciones);
                        jugador.mapa.actualizar_posicion_barco(&posicion_inicial, &nuevas_posiciones, jugador.id);
                        break;
                    }
                }
            }
        }
    }
    /// Función que procesa un ataque en el mapa
    /// 
    /// # Args
    /// 
    /// `coordenadas_ataque` - Coordenadas del ataque
    /// 
    /// `jugador_id` - ID del jugador que realiza el ataque
    /// 
    /// `jugadores` - Vector de jugadores
    /// 
    /// `mapa` - Mapa en el que se realiza el ataque
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador con el ataque procesado
    fn procesar_ataque(coordenadas_ataque: (i32, i32), jugador_id: usize, jugadores: &mut Vec<Jugador>) {
        let mut puntos_ganados = 0;
        for jugador in jugadores.iter_mut() {
            let mut jugador_atacante = jugador.clone();
            if jugador.id != jugador_id {
                let puntos = jugador.procesar_ataque(coordenadas_ataque, &mut jugador_atacante.mapa);
                puntos_ganados += puntos;
                if puntos > 0 {
                    jugador.mapa.marcar_hundido(coordenadas_ataque);
                }
            
            }

        }
        for jugador in jugadores.iter_mut(){
            if jugador.id == jugador_id {
                jugador.puntos += puntos_ganados;
            }
        }
    }
}
