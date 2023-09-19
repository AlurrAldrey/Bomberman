use crate::Bomb;

///representa la respuesta que devuelve la matriz al main cuando se afecta un casillero
#[derive(PartialEq, Debug)]
pub enum AffectResponse {
    Deviate { direction: char },
    Explode { bomb: Bomb },
    Stop,
    Continue,
    AffectError { err: String },
}
