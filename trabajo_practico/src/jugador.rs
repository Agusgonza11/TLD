use std::io;
use crate::{acciones::{Accion, Ataque, Movimiento}, barco::Barco, mapa::Mapa};

const ATAQ: &str = "atacar";
const MOV: &str = "mover";

pub struct Jugador{
    pub id: usize,
    pub barcos: Vec<Barco>
}

impl Jugador{
    pub fn new(id: usize, mapa: &mut Mapa) -> Jugador {
        let mut barcos = Vec::new();
        let posicion_libre = mapa.obtener_posicion_libre(id.to_string());
        barcos.push(Barco::new("basico".to_string(), 1, posicion_libre));
        Jugador { id, barcos }
    }

    pub fn turno(&mut self, tablero: &Mapa) -> Accion {
        tablero.imprimir_tablero(self.id.to_string());

        let mut accion = String::new();
        io::stdin().read_line(&mut accion)
            .expect("Error al leer la entrada");
    
        match accion.trim() {
            "m" => self.moverse(),
            "a" => self.atacar(),
            "t" => Accion::Saltar,
            _ => Accion::Saltar,
        }
    }

    fn atacar(&self) -> Accion {
        let (_barco_seleccionado, _cordenadas_atacadas) = self.pedir_instrucciones(ATAQ);
        let id_barco = 0;
        let cordenadas_ataque = (0,0);
        return Accion::Atacar(Ataque { jugador_id: self.id, id_barco, cordenadas_ataque })
    }

    fn moverse(&self) -> Accion {
        let (_barco_seleccionado, _cordenadas_atacadas) = self.pedir_instrucciones(MOV);
        let id_barco = 0;
        let cordenadas_origen = (0,0);
        let cordenadas_destino = (0,1);
        return Accion::Moverse(Movimiento { jugador_id: self.id, id_barco, cordenadas_origen, cordenadas_destino })
    }

    fn pedir_instrucciones(&self, accion: &str) -> (String, String) {
        let mut barco_seleccionado = String::new();
        println!("Elige un barco para {}: {:?}", accion, self.barcos);
        io::stdin().read_line(&mut barco_seleccionado)
            .expect("Error al leer la entrada");
        let mut movimiento_seleccionado = String::new();
        println!("Elige una direccion para {}: ", accion);
        io::stdin().read_line(&mut movimiento_seleccionado)
            .expect("Error al leer la entrada");
        return (barco_seleccionado, movimiento_seleccionado)
    }

    fn pedir_movimiento(&self) {

    }

    fn pedir_ataque(&self) {
        
    }

}