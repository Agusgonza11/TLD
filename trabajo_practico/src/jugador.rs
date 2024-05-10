use std::io;

pub struct Jugador{
    pub id: i32
}

impl Jugador{
    pub fn new(id: i32) -> Jugador {
        Jugador { id }
    }

    pub fn turno(&mut self) {
        let mut accion = String::new();
        io::stdin().read_line(&mut accion)
            .expect("Error al leer la entrada");
    
        match accion.trim() {
            "m" => println!("te moviste"),
            "a" => println!("atacaste"),
            "t" => println!("tienda"),
            _ => println!("Ingrese una accion valida"),
        }
    }
}