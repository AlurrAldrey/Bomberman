use crate::bomb::Bomb;

#[derive(Clone, Debug)]
pub struct Enemy {
    pub id: String,
    pub health: u32,
    pub hitted_by: Vec<(u32, u32)>, //las coordenadas de las bombas que lo golpearon
}

impl Enemy {
    //devuelve true si mató al enemigo, false si no lo mató
    pub fn damage(&mut self, bomb: &Bomb) -> Option<bool> {
        if !self.is_hittable_by(bomb) {
            return Some(false);
        }

        self.hitted_by.push(bomb.position);
        let result = self.health.checked_sub(1);
        match result {
            Some(new_health) => {
                self.health = new_health;
                return Some(self.health == 0);
            }
            None => return None, //si ya estaba muerto
        }
    }

    fn is_hittable_by(&self, bomb: &Bomb) -> bool {
        return !self.hitted_by.contains(&bomb.position);
    }
}
