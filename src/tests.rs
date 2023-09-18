#[cfg(test)]
mod tests {

    use crate::affect_response::AffectResponse;
    //Bomb y Enemy
    use crate::bomb::{Bomb, BombType};
    use crate::enemy::Enemy;

    #[test]
    fn test_damage_enemy_alive() {
        let mut enemy = Enemy {
            id: "F".to_string(),
            health: 3,
            hitted_by: vec![],
        };

        let bomb = Bomb {
            id: "B".to_string(),
            range: 1,
            position: (0, 0),
            bomb_type: BombType::Normal
        };

        let result = enemy.damage(&bomb);

        assert_eq!(result, Some(false)); //no murió
        assert_eq!(enemy.health, 2); //perdió 1 de vida
        assert_eq!(enemy.hitted_by, vec![(0, 0)]); //agrego la bomba a hitted_by
    }

    #[test]
    fn test_damage_dead_enemy() {
        let mut enemy = Enemy {
            id: "F".to_string(),
            health: 0,
            hitted_by: vec![],
        };

        let bomb = Bomb {
            id: "B".to_string(),
            range: 1,
            position: (1, 1),
            bomb_type: BombType::Normal
        };

        let result = enemy.damage(&bomb);

        assert_eq!(result, None); // Devuelve None porque ataca a un muerto
        assert_eq!(enemy.health, 0); // No le resta vida
        assert_eq!(enemy.hitted_by, vec![]); // No le pega la bomba
    }

    #[test]
    fn test_enemy_is_not_hittable_twice_by_same_bomb() {
        let mut enemy = Enemy {
            id: "F".to_string(),
            health: 3,
            hitted_by: vec![],
        };

        let bomb = Bomb {
            id: "B".to_string(),
            range: 1,
            position: (0, 0),
            bomb_type: BombType::Normal
        };

        enemy.damage(&bomb); //lo ataco una vez
        let result = enemy.damage(&bomb); //2da vez con la misma bomba

        assert_eq!(result, Some(false)); //la segunda vez no le pegó
        assert_eq!(enemy.health, 2); //perdió 1 de vida
        assert_eq!(enemy.hitted_by, vec![(0, 0)]); //agrego la bomba a hitted_by 1 vez
    }

    #[test]
    fn test_enemy_is_hittable_twice_by_different_bombs() {
        let mut enemy = Enemy {
            id: "F".to_string(),
            health: 3,
            hitted_by: vec![],
        };

        let bomb1 = Bomb {
            id: "B".to_string(),
            range: 1,
            position: (0, 0),
            bomb_type: BombType::Normal
        };

        let bomb2 = Bomb {
            id: "B".to_string(),
            range: 1,
            position: (1, 1),
            bomb_type: BombType::Normal
        };

        enemy.damage(&bomb1); //lo ataco una vez
        let result = enemy.damage(&bomb2); //2da vez con la misma bomba

        assert_eq!(result, Some(false)); //no lo mata y le pega
        assert_eq!(enemy.health, 1); //perdió 2 de vida
        assert_eq!(enemy.hitted_by, vec![(0, 0),(1,1)]); //agrego las bombas a hitted_by
    }
    
    //Helpers
    use crate::helpers::increment_burst_position;

    #[test]
    fn test_increment_burst_position_up() {
        let direction = 'U';
        let position = (2, 2);
        let i = 1;
        let matrix_dimension = &5;

        let result = increment_burst_position(direction, position, i, matrix_dimension);

        assert_eq!(result, Some((2, 1)));//"disminuye" en Y porque la matriz en el archivo se muestra de arriba hacia abajo
    }

    #[test]
    fn test_increment_burst_position_right() {
        let direction = 'R';
        let position = (2, 2);
        let i = 1;
        let matrix_dimension = &5;

        let result = increment_burst_position(direction, position, i, matrix_dimension);

        assert_eq!(result, Some((3, 2)));//aumenta en X
    }

    #[test]
    fn test_increment_burst_position_down() {
        let direction = 'D';
        let position = (2, 2);
        let i = 1;
        let matrix_dimension = &5;

        let result = increment_burst_position(direction, position, i, matrix_dimension);

        assert_eq!(result, Some((2, 3)));//"aumenta" en Y porque la matriz en el archivo se muestra de arriba hacia abajo
    }

    #[test]
    fn test_increment_burst_position_left() {
        let direction = 'L';
        let position = (2, 2);
        let i = 1;
        let matrix_dimension = &5;

        let result = increment_burst_position(direction, position, i, matrix_dimension);

        assert_eq!(result, Some((1, 2)));//disminuye en X
    }

    #[test]
    fn test_increment_overflow() {
        let direction = 'R';
        let position = (2, 2);
        let i = 1;
        let matrix_dimension = &3;

        let result = increment_burst_position(direction, position, i, matrix_dimension);

        assert_eq!(result, None);//supera la dimension de la matriz en X
    }

    #[test]
    fn test_increment_underflow() {
        let direction = 'L';
        let position = (0, 2);
        let i = 1;
        let matrix_dimension = &3;

        let result = increment_burst_position(direction, position, i, matrix_dimension);

        assert_eq!(result, None);//es menor que 0 en X
    }

    #[test]
    fn test_increment_invalid_direction() {
        let direction = 'Z';//direccion invalida
        let position = (0, 2);
        let i = 1;
        let matrix_dimension = &3;

        let result = increment_burst_position(direction, position, i, matrix_dimension);

        assert_eq!(result, None);
    }

    //MapObject
    use crate::map_object::MapObject;

    #[test]
    fn test_create_shredding_bomb() {
        let result = MapObject::new("S1", (0,0));

        match result {
            Some(MapObject::Bomb(bomb)) => {
                assert_eq!(bomb.id, "S");
                assert_eq!(bomb.range, 1);
                assert_eq!(bomb.bomb_type, BombType::Shredding);
                assert_eq!(bomb.position, (0,0));
            },
            _ => panic!("Esperaba un MapObject::Bomb")
        }
    }

    #[test]
    fn test_create_normal_bomb() {
        let result = MapObject::new("B1", (0,0));

        match result {
            Some(MapObject::Bomb(bomb)) => {
                assert_eq!(bomb.id, "B");
                assert_eq!(bomb.range, 1);
                assert_eq!(bomb.bomb_type, BombType::Normal);
                assert_eq!(bomb.position, (0,0));
            },
            _ => panic!("Esperaba un MapObject::Bomb")
        }
    }

    #[test]
    fn test_create_bomb_invalid_range() {
        let result = MapObject::new("BX", (0,0));

        assert!(result.is_none());
    }

    #[test]
    fn test_create_enemy() {
        let result = MapObject::new("F1", (0,0));

        match result {
            Some(MapObject::Enemy(enemy)) => {
                assert_eq!(enemy.id, "F");
                assert_eq!(enemy.health, 1);
            },
            _ => panic!("Esperaba un MapObject::Enemy")
        }
    }

    #[test]
    fn test_create_enemy_invalid_health() {
        let result = MapObject::new("FX", (0,0));

        assert!(result.is_none());
    }

    #[test]
    fn test_create_deviation() {
        let result = MapObject::new("DU", (0,0));

        match result {
            Some(MapObject::Deviation{ id, direction }) => {
                assert_eq!(id, "D");
                assert_eq!(direction, 'U');
            },
            _ => panic!("Esperaba un MapObject::Deviation")
        }
    }

    #[test]
    fn test_create_wall() {
        let result = MapObject::new("W", (0,0));

        match result {
            Some(MapObject::Wall{ id }) => {
                assert_eq!(id, "W");
            },
            _ => panic!("Esperaba un MapObject::Wall")
        }
    }

    #[test]
    fn test_create_rock() {
        let result = MapObject::new("R", (0,0));

        match result {
            Some(MapObject::Rock{ id }) => {
                assert_eq!(id, "R");
            },
            _ => panic!("Esperaba un MapObject::Rock")
        }
    }

    #[test]
    fn test_create_invalid_object() {
        let result = MapObject::new("X1", (0,0));

        assert!(result.is_none());
    }

    //Matrix
    use crate::Matrix;
    use crate::Burst;

    #[test]
    fn test_affect_position_nothing() {
        let mut matrix = Matrix::new(String::from("_"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb::new(position);
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        
        assert_eq!(result, AffectResponse::Continue); //no la afecta y sigue
    }

    #[test]
    fn test_affect_position_wall() {
        let mut matrix = Matrix::new(String::from("W"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb::new(position);
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        
        assert_eq!(result, AffectResponse::Stop); //no la afecta y frena
    }

    #[test]
    fn test_affect_position_enemy_continues() {
        let mut matrix = Matrix::new(String::from("F2"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb::new(position);
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        let enemy = matrix.get(position.0, position.1);
        match enemy {
            MapObject::Enemy(enemy) => {
                assert_eq!(enemy.health, 1);
                assert_eq!(result, AffectResponse::Continue); //continua
            },
            _ => panic!("Se esperaba encontrar un enemigo")
        }
    }

    #[test]
    fn test_affect_position_enemy_killed() {
        let mut matrix = Matrix::new(String::from("F1"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb::new(position);
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        let enemy = matrix.get(position.0, position.1);
        match enemy {
            MapObject::Nothing { id } => {
                assert_eq!(id, "_"); //reemplaza al enemigo por un Nothing
                assert_eq!(result, AffectResponse::Continue); //continua
            },
            _ => panic!("Se esperaba encontrar un Nothing")
        }
    }

    #[test]
    fn test_affect_position_rock_w_normal_bomb() {
        let mut matrix = Matrix::new(String::from("R"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb{ id: String::from("B"), range: 1, bomb_type: BombType::Normal, position};
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        
        assert_eq!(result, AffectResponse::Stop); //frena
    }

    #[test]
    fn test_affect_position_rock_w_shredding_bomb() {
        let mut matrix = Matrix::new(String::from("R"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb{ id: String::from("S"), range: 1, bomb_type: BombType::Shredding, position};
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        
        assert_eq!(result, AffectResponse::Continue); //sigue de largo
    }

    #[test]
    fn test_affect_position_deviation() {
        let mut matrix = Matrix::new(String::from("DU"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb::new(position);
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        match result {
            AffectResponse::Deviate { direction } => {
                assert_eq!(direction, 'U'); //devuelve la direccion
            },
            _ => panic!("Se esperaba encontrar un Deviate")
        }
    }

    #[test]
    fn test_affect_position_bomb() {
        let mut matrix = Matrix::new(String::from("B1"), &mut String::from("")).unwrap();
        let position = (0,0);
        let bomb = Bomb::new(position);
        let result = matrix.affect_position(
            position,
            &Burst::new('U', position, 0, bomb),
        );

        let nothing = matrix.get(position.0, position.1);
        match result {
            AffectResponse::Explode	 { bomb } => { //la explota
                assert_eq!(bomb.range, 1); //devuelve el rango
                assert_eq!(bomb.position, (0,0)); //devuelve el rango
                assert_eq!(bomb.bomb_type, BombType::Normal); //devuelve el tipo
                assert_eq!(nothing, MapObject::Nothing { id: String::from("_") }); //devuelve el rango
            },
            _ => panic!("Se esperaba encontrar un Explode")
        }
    }
    
}