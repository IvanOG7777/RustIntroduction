struct Weapon {
    name: String,
    damage: f32
}
struct Player {
    name: String,
    health: f32,
    weapon: Option<Weapon>,
}

impl Player {
    fn create_player(name: String) -> Player {
        Player {
            name,
            health: 100.0,
            weapon: None,
        }
    }

    fn print_player_stats(&self) {
        println!("Name: {}", self.name);
        println!("Health: {}", self.health);
        match &self.weapon {
            Some(weapon) => {
                println!("Weapon: {}, damage: {}", weapon.name, weapon.damage);
            }

            None => println!("No weapon equipped"),
        }
    }

    fn attack(&self) {
        match &self.weapon {
            Some(weapon) => {
                println!("{} attacks with {} for {} damage", self.name, weapon.name, weapon.damage);
            }

            None => println!("No weapon equipped"),
        }
    }

    fn equip_weapon(&mut self, name: String, damage: f32) {
        let weapon = Weapon {name, damage};

        self.weapon = Some(weapon);
    }
}


fn main() {
    let mut player1 = Player::create_player(String::from("Ivan"));

    player1.print_player_stats();

    player1.attack();

    player1.equip_weapon(String::from("Sword"), 30.0);

    player1.print_player_stats();
    player1.attack();
}