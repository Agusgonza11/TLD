use juego::cliente::Cliente;
use libreria::custom_error::CustomError;

fn main() -> Result<(), CustomError> {
    let addr = "127.0.0.1:8080".to_owned();
    let mut cliente = match Cliente::new(addr, 1.to_owned(), "".to_string()) {
        Ok(cliente) => cliente,
        Err(_) => {
            return Err(CustomError::ErrorCreatingSocket);
        }
    };
    println!("Nueva sesión iniciada");
    cliente.run()?;

    Ok(())
}
