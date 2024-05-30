use crate::{
    accion::Accion, ataque::Ataque, barco::Barco, estado_barco::EstadoBarco, mapa::Mapa,
    movimiento::Movimiento,
};
use std::io::{self, Write};

use crate::constantes::{ATAQ, MOV};

#[derive(Clone)]
pub struct Jugador {
    pub id: usize,
    pub barcos: Vec<Barco>,
    pub puntos: usize,
    pub monedas: usize,
}

impl Jugador {
    /// Función que crea un nuevo jugador
    /// 
    /// # Args
    /// 
    /// `id` - Identificador del jugador
    /// 
    /// `mapa` - Mapa en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// `Jugador` - Jugador creado
    pub fn new(id: usize, mapa: &mut Mapa) -> Jugador {
        let mut barcos = Vec::new();
        let mut id_actual = 0;

        let tamaños_barcos = vec![3];

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
            monedas: 500,
        }
    }
    /// Función que permite al jugador realizar un turno
    /// 
    /// # Args
    /// 
    /// `tablero` - Mapa en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// `Accion` - Acción realizada por el jugador
    pub fn turno(&mut self, tablero: &Mapa) -> Accion {
        tablero.imprimir_tablero(self.id.to_string());

        loop {
            let mut accion = String::new();
            println!("Elige una acción (m: moverse, a: atacar, t: abrir la tienda, s:saltar): ");
            io::stdin()
                .read_line(&mut accion)
                .expect("Error al leer la entrada");

            match accion.trim() {
                "m" => return self.moverse(tablero),
                "a" => return self.atacar(),
                "t" => return self.abrir_tienda(),
                "s" => return Accion::Saltar,
                _ => println!("Error en la accion. Por favor, elige una acción valida (m, a, t, s)."),
            }
        }
    }
    /// Función que permite al jugador atacar a un barco enemigo
    /// 
    /// # Returns
    /// 
    /// `Accion` - Acción de ataque realizada por el jugador
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
    /// Función que permite al jugador moverse en el tablero
    ///
    /// # Args
    /// 
    /// `mapa` - Mapa en el que se moverá el jugador
    /// 
    /// # Returns
    /// 
    /// `Accion` - Acción de movimiento realizada por el jugador
    fn moverse(&self, mapa: &Mapa) -> Accion {
        let (barco_seleccionado, coordenadas_destino) = self.pedir_instrucciones(MOV);
        let id_barco = barco_seleccionado.id;
        let coordenadas_origen = barco_seleccionado.posiciones[0];
    
        let tamano_barco = barco_seleccionado.tamaño;
    
        let coordenadas_contiguas = mapa.obtener_coordenadas_contiguas(coordenadas_destino, tamano_barco);
    
        if coordenadas_contiguas.is_empty() {
            println!("No hay suficientes espacios contiguos disponibles para mover el barco.");
            return self.moverse(mapa);
        }
    
        let mut nuevas_posiciones = vec![];
        for (i, &coordenada) in coordenadas_contiguas.iter().enumerate() {
            nuevas_posiciones.push(coordenada);
            if i == tamano_barco - 1 {
                break;
            }
        }
    
        return Accion::Moverse(Movimiento {
            jugador_id: self.id,
            id_barco,
            coordenadas_origen,
            cordenadas_destino: nuevas_posiciones,
        });
    }
    
    /// Función que permite al jugador seleccionar un barco y coordenadas para realizar una acción
    /// 
    /// # Args
    /// 
    /// `accion` - Acción que se realizará con el barco seleccionado
    /// 
    /// # Returns
    /// 
    /// `(Barco, (i32, i32))` - Tupla con el barco seleccionado y las coordenadas de la acción
    fn pedir_instrucciones(&self, accion: &str) -> (Barco, (i32, i32)) {
        println!("Elige un barco para {}:", accion);
        for (i, barco) in self.barcos.iter().enumerate() {
            println!("{}: ID: {}, Posición: {:?}", i, barco.id, barco.posiciones);
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
    /// Función que permite al jugador ingresar coordenadas
    /// 
    /// # Returns
    /// 
    /// `(i32, i32)` - Tupla con las coordenadas ingresadas por el jugador
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
    /// Función que procesa un ataque realizado por un jugador
    /// 
    /// # Args
    /// 
    /// `coordenadas_ataque` - Coordenadas del ataque realizado por el jugador
    /// 
    /// `mapa` - Mapa en el que se encuentra el jugador
    /// 
    /// # Returns
    /// 
    /// `()` - No retorna nada
    pub fn procesar_ataque(&mut self, coordenadas_ataque: (i32, i32), mapa: &mut Mapa) {
        let mut barcos_golpeados = false;
        let mut barcos_hundidos = Vec::new();

        for barco in &mut self.barcos {
            let mut todas_las_partes_hundidas = true; 
            for &posicion in &barco.posiciones {
                if posicion == coordenadas_ataque {
                    barcos_golpeados = true;
                    match barco.estado {
                        EstadoBarco::Sano => {
                            println!("Le pegaste a un barco");
                            barco.tamaño -= 1;
                            if barco.tamaño == 0 {
                                barco.estado = EstadoBarco::Hundido;
                                println!("El barco ha sido hundido");
                                println!("Ganaste 15 puntos");
                                self.puntos += 15;
                                barcos_hundidos.extend(barco.posiciones.iter().cloned()); 
                            } else {
                                barco.estado = EstadoBarco::Golpeado;
                                println!("Ganaste 5 puntos");
                                self.puntos += 5;
                            }
                        }
                        EstadoBarco::Golpeado => {
                            println!("Le pegaste a un barco de nuevo");
                            barco.tamaño -= 1;
                            if barco.tamaño == 0 {
                                barco.estado = EstadoBarco::Hundido;
                                println!("El barco ha sido ha sido hundido");
                                println!("Ganaste 10 puntos");
                                self.puntos += 10;
                                barcos_hundidos.extend(barco.posiciones.iter().cloned()); 
                            } else {
                                println!("Ganaste 5 puntos");
                                self.puntos += 5;
                            }
                        }
                        EstadoBarco::Hundido => {
                            println!("El barco ya ha sido hundido");
                        }
                    }
                } else {
                    todas_las_partes_hundidas = false; 
                }
            }
            if todas_las_partes_hundidas {
                barco.estado = EstadoBarco::Hundido; 
            }
        }

        self.barcos.retain(|barco| barco.estado != EstadoBarco::Hundido);

        for coordenadas in barcos_hundidos {
            mapa.marcar_hundido(coordenadas);
        }

        if !barcos_golpeados {
            println!("No le pegaste a nada, burro irrecuperable.");
        }
    }
    
}
