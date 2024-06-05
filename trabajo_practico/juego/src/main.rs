
use juego::server::Server;

fn main() {
    let mut server = match Server::new() {
        Ok(server) => server,
        Err(err) => {
            eprintln!("Error al iniciar el servidor: {:?}", err);
            return;
        }
    };

    if let Err(err) = server.run() {
        eprintln!("Error al ejecutar el servidor: {:?}", err);
    }
}