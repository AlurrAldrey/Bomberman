use crate::Bomb;
pub enum AffectResponse {
    Deviate { direction: char },
    Explode { bomb: Bomb },
    Stop,
    Continue,
    AffectError { err: String },
}
