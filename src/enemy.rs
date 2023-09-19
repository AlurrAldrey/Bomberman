use crate::bomb::Bomb;

///representa a un enemigo
#[derive(Clone, Debug, PartialEq)]
pub struct Enemy {
    pub id: String,
    pub health: u32,
    pub hitted_by: Vec<(u32, u32)>, //las coordenadas de las bombas que lo golpearon
}

impl Enemy {
    ///devuelve true si mató al enemigo, false si no lo mató
    pub fn damage(&mut self, bomb: &Bomb) -> Option<bool> {
        if !self.is_hittable_by(bomb) {
            return Some(false);
        }
        let result = self.health.checked_sub(1);
        match result {
            Some(new_health) => {
                self.hitted_by.push(bomb.position);
                self.health = new_health;
                Some(self.health == 0)
            }
            None => None, //si ya estaba muerto
        }
    }

    ///define si el enemigo puede ser atacado o no por una bomba
    fn is_hittable_by(&self, bomb: &Bomb) -> bool {
        !self.hitted_by.contains(&bomb.position)
    }
}
