use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    Err,
    AccionInvalida,
    ErrorCreatingSocket,
    ErrorAceptandoConexion,
    ErrorEnviandoInstruccion,
    ErrorRecibiendoInstruccion,
    ErrorJugadorInexistente,
    ErrorParseandoInstruccion,
    LongitudNombreInvalida,
    ErrorEnviarMensaje,
    ErrorSerializacion,
    

}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::Err => write!(f, ""),
            CustomError::AccionInvalida=> write!(f, "Error: Acción invalida"),
            CustomError::ErrorCreatingSocket => write!(f, "Error al crear el socket"),
            CustomError::ErrorAceptandoConexion => write!(f, "Error al aceptar la conexión"),
            CustomError::ErrorEnviandoInstruccion => write!(f, "Error al enviar la instrucción"),
            CustomError::ErrorRecibiendoInstruccion => write!(f, "Error al recibir la instrucción"),
            CustomError::ErrorJugadorInexistente => write!(f, "Error: Jugador inexistente"),
            CustomError::ErrorParseandoInstruccion => write!(f, "Error al parsear la instrucción"),
            CustomError::LongitudNombreInvalida => write!(f, "Error: Longitud de nombre invalida"),
            CustomError::ErrorEnviarMensaje => write!(f, "Error al enviar el mensaje"),
            CustomError::ErrorSerializacion => write!(f, "Error al serializar"),
        }
    }
}
