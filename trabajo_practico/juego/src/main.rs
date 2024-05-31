use std::io;
use std::io::Write;

use juego::juego::Juego;

fn main() {
    print!("Elige el número de jugadores: ");
    io::stdout().flush().expect("Error");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error");

    let num_jugadores: usize = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("El argumento debe ser un número entero válido.");
            std::process::exit(1);
        }
    };

    let mut juego = Juego::new(num_jugadores);
    let _ = juego.iniciar_juego();
}
