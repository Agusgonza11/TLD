use acciones::{accion::Accion, ataque::Ataque, movimiento::Movimiento};
use barcos::{barco::Barco, estado_barco::EstadoBarco};
use libreria::{constantes::{ATAQ, MOV}, custom_error::CustomError};

use crate::{ mapa::Mapa, server::Server}  ;
use std::{io::{self, Write}, net::TcpStream, vec};


#[derive(Clone)]
pub struct Jugador {
    pub id: usize,
    pub mapa: Mapa,
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

        let tamaños_barcos: Vec<usize> = vec![5,4,3];
        for tamaño in tamaños_barcos {
            let vec_posiciones = mapa.obtener_posiciones_libres_contiguas(id.to_string(), tamaño);
            let id_barco = id_actual;
            id_actual += 1;
            barcos.push(Barco::new(id_barco, tamaño, vec_posiciones));
        }

        Jugador {
            id,
            barcos,
            puntos: 0,
            monedas: 500,
            mapa: mapa.clone(),
        }
    }

    pub fn enviar_instrucciones(&self, server: &mut Server) -> Result<(), CustomError> {
        let mensaje = format!("Puntos:{}:\nElige una acción (m: moverse, a: atacar, t: abrir la tienda, s: saltar)", self.puntos);
    
        if let Some(conexion) = server.conexiones_jugadores.get(&self.id) {
            let conexion = conexion.lock().map_err(|_| CustomError::ErrorAceptandoConexion)?;
            Self::enviar_mensaje(&conexion, mensaje.as_bytes().to_vec())?;
        }
    
        Ok(())
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
    pub fn turno(&mut self,server:&mut Server) -> Accion {
        self.mapa.imprimir_tablero(self.id.to_string());
        let _ = self.mapa.enviar_tablero(self.id.to_string(), server);
        loop {
            let _ = self.enviar_instrucciones(server);
            let mut accion = String::new();
            println!("Puntos: {}", self.puntos);
            //println!("Elige una accion (m: moverse, a: atacar, t: abrir la tienda, s:saltar): ");
            io::stdin()
                .read_line(&mut accion)
                .expect("Error al leer la entrada");

            //Aca tengo que recibir la accion de parte del cliente
            match accion.trim() {
                "m" => return self.moverse(server),
                "a" => return self.atacar(),
                "t" => return self.abrir_tienda(),
                "s" => return Accion::Saltar,
                _ => println!("Error en la accion. Por favor, elige una accion valida (m, a, t, s)."),
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
    fn moverse(&mut self,server: &mut Server) -> Accion {
        
        let (barco_seleccionado, coordenadas_destino) = self.pedir_instrucciones(MOV);
        if barco_seleccionado.estado == EstadoBarco::Golpeado || barco_seleccionado.estado == EstadoBarco::Hundido{
            println!("El barco seleccionado esta golpeado, no se puede mover, elija otra accion u otro barco.");
            return self.turno(server);
         }
        let id_barco = barco_seleccionado.id;
        let coordenadas_origen = barco_seleccionado.posiciones[0];
        
        let coordenadas_contiguas = self.mapa.obtener_coordenadas_contiguas(coordenadas_destino,barco_seleccionado.tamaño);
        

        if coordenadas_contiguas.is_empty() {
            println!("No hay suficientes espacios contiguos disponibles para mover el barco.");
            return self.moverse(server);
        }
    
        let mut nuevas_posiciones = vec![];
        for (i, &coordenada) in coordenadas_contiguas.iter().enumerate() {
            nuevas_posiciones.push(coordenada);
            if i == barco_seleccionado.tamaño - 1 {
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
            println!("{}: ID: {}, Posicion: {:?}", i, barco.id, barco.posiciones);
        }

        let mut barco_seleccionado = String::new();
        io::stdout().flush().expect("Error");
        io::stdin()
            .read_line(&mut barco_seleccionado)
            .expect("Error");
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
    /// `usize` - Puntos ganados por el jugador
    
    pub fn procesar_ataque(&mut self, coordenadas_ataque: (i32, i32), mapa: &mut Mapa) -> usize{
        let mut puntos = 0;
        let mut barcos_golpeados = false;
        let mut barcos_hundidos = Vec::new();

        for barco in &mut self.barcos {
            if barco.posiciones.contains(&coordenadas_ataque) {
                barcos_golpeados = true;
                barco.posiciones.retain(|&pos| pos != coordenadas_ataque); 

                if barco.posiciones.is_empty() {
                    barco.estado = EstadoBarco::Hundido;
                    println!("El barco ha sido hundido");
                    println!("Ganaste 15 puntos");
                    puntos += 15;
                    barcos_hundidos.push(coordenadas_ataque);

                } else {
                    if barco.estado == EstadoBarco::Sano {
                        barco.estado = EstadoBarco::Golpeado;
                        println!("Le pegaste a un barco");
                        println!("Ganaste 5 puntos");
                        puntos += 5;
                    } else if barco.estado == EstadoBarco::Golpeado {
                        println!("Ganaste 5 puntos");
                        puntos += 5;
                    }
                }
            }
        }

        self.barcos.retain(|barco| barco.estado != EstadoBarco::Hundido);

        for coordenadas in barcos_hundidos {
            mapa.marcar_hundido(coordenadas);
        }

        if !barcos_golpeados {
            println!("No le pegaste a nada, burro irrecuperable.");
        }
        puntos
    }
    fn enviar_mensaje(mut stream: &TcpStream, msg: Vec<u8>) -> Result<(), CustomError> {
        let result_stream = stream.write_all(&msg);
        result_stream.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        let result_flush = stream.flush();
        result_flush.map_err(|_| CustomError::ErrorEnviarMensaje)?;
        Ok(())
    }
}