use crate::Bomb;

#[derive(PartialEq, Debug)]
pub enum AffectResponse {
    Deviate { direction: char },
    Explode { bomb: Bomb },
    Stop,
    Continue,
    AffectError { err: String },
}
