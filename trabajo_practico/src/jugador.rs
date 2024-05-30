use crate::{
    accion::Accion, ataque::Ataque, barco::Barco,  estado_barco::EstadoBarco, mapa::Mapa, movimiento::Movimiento
};
use std::io::{self, Write};

const ATAQ: &str = "atacar";
const MOV: &str = "mover";

#[derive(Clone)]
pub struct Jugador {
    pub id: usize,
    pub barcos: Vec<Barco>,
    pub puntos: usize,
}

impl Jugador {
    pub fn new(id: usize, mapa: &mut Mapa) -> Jugador {
        let mut barcos = Vec::new();
        let mut id_actual = 0;

        let tamaños_barcos = vec![1, 1, 3, 3, 5, 5];

        for tamaño in tamaños_barcos {
            let posicion_libre = mapa.obtener_posicion_libre(id.to_string());
            let id_barco = id_actual;
            id_actual += 1;
            barcos.push(Barco::new(id_barco, tamaño, posicion_libre));
        }

        Jugador {
            id,
            barcos,
            puntos: 0,
        }
    }

    pub fn turno(&mut self, tablero: &Mapa) -> Accion {
        tablero.imprimir_tablero(self.id.to_string());

        loop {
            let mut accion = String::new();
            println!("Elige una acción (m: moverse, a: atacar, t: abrir la tienda, s:saltar): ");
            io::stdin()
                .read_line(&mut accion)
                .expect("Error al leer la entrada");

            match accion.trim() {
                "m" => return self.moverse(),
                "a" => return self.atacar(),
                "t" => return self.abrir_tienda(),
                "s" => return Accion::Saltar,
                _ => println!("Error en la accion. Por favor, elige una acción valida (m, a, t, s)."),
            }
        }
    }

    fn atacar(&self) -> Accion {
        let (barco_seleccionado, cordenadas_atacadas) = self.pedir_instrucciones(ATAQ);
        let id_barco = barco_seleccionado.id;
        let cordenadas_ataque = cordenadas_atacadas;
        return Accion::Atacar(Ataque {
            jugador_id: self.id,
            id_barco,
            cordenadas_ataque,
        });
    }

    fn moverse(&self) -> Accion {
        let (barco_seleccionado, coordenadas_destino) = self.pedir_instrucciones(MOV);
        let id_barco = barco_seleccionado.id;
        let cordenadas_origen = barco_seleccionado.posicion;
        let cordenadas_destino = coordenadas_destino;
        return Accion::Moverse(Movimiento {
            jugador_id: self.id,
            id_barco,
            cordenadas_origen,
            cordenadas_destino,
        });
    }

    fn pedir_instrucciones(&self, accion: &str) -> (Barco, (i32, i32)) {
        println!("Elige un barco para {}:", accion);
        for (i, barco) in self.barcos.iter().enumerate() {
            println!("{}: ID: {}, Posición: {:?}", i, barco.id, barco.posicion);
        }

        let mut barco_seleccionado = String::new();
        io::stdout().flush().expect("Error al limpiar stdout");
        io::stdin()
            .read_line(&mut barco_seleccionado)
            .expect("Error al leer la entrada");
        let barco_seleccionado: usize = match barco_seleccionado.trim().parse() {
            Ok(numero) => numero,
            Err(_) => {
                println!("Numero de barco invalido. Por favor, ingrese un numero dentro del rango.");
                return self.pedir_instrucciones(accion);
            }
        };

        if barco_seleccionado >= self.barcos.len() {
            println!("Numero de barco invalido. Por favor, elige un numero dentro del rango.");
            return self.pedir_instrucciones(accion);
        }

        let barco = self.barcos[barco_seleccionado].clone();
        let coordenadas = Self::pedir_coordenadas();

        (barco, coordenadas)
    }

    fn pedir_coordenadas() -> (i32, i32) {
        loop {
            println!("Ingresa las coordenadas en formato 'x,y': ");

            let mut coordenadas = String::new();
            io::stdin()
                .read_line(&mut coordenadas)
                .expect("Error al leer la entrada");

            let mut iter = coordenadas.trim().split(',');
            if let (Some(x_str), Some(y_str)) = (iter.next(), iter.next()) {
                if let Ok(x) = x_str.trim().parse::<i32>() {
                    if let Ok(y) = y_str.trim().parse::<i32>() {
                        return (x, y);
                    }
                }
            }

            println!("Formato de coordenadas incorrecto. Intentalo de nuevo.");
        }
    }

    fn abrir_tienda(&self) -> Accion {
        println!("Tienda abierta");
        Accion::Tienda(self.puntos)
    }

    pub fn procesar_ataque(&mut self, coordenadas_ataque: (i32, i32),mapa:&mut Mapa) {
        let mut barcos_golpeados = 0;
        for barco in &mut self.barcos {
            if barco.posicion == coordenadas_ataque {
                barcos_golpeados += 1;
                match barco.estado {
                    EstadoBarco::Sano => {
                        println!("Le pegaste a un barco");
                        barco.tamaño -= 1;
                        if barco.tamaño == 0 {
                            barco.estado = EstadoBarco::Hundido;
                            println!("El barco ha sido hundido");
                            println!("Ganaste 15 puntos");
                            self.puntos += 5;
                            mapa.marcar_hundido(barco.posicion);

                        } else {
                            barco.estado = EstadoBarco::Golpeado;
                            print!("Ganaste 5 puntos");
                            self.puntos += 5;

                            
                        }
                    }
                    EstadoBarco::Golpeado => {
                        println!("Le pegaste a un barco de nuevo");
                        barco.tamaño -= 1;
                 
                        if barco.tamaño == 0 {
                            println!("El barco ha sido hundido");
                            println!("Ganaste 10 puntos");
                            barco.estado = EstadoBarco::Hundido;
                            self.puntos += 10;
                        }
                        self.puntos += 5;
                        println!("Ganaste 5 puntos");
                    }
                    EstadoBarco::Hundido => {
                        println!("El barco ya ha sido hundido");
                    }
                }
            }
        }
        if barcos_golpeados == 0{
            println!("No le pegaste a nada burro irrecuperable");
    }
    }
}