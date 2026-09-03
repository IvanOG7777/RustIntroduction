struct Player {
    name: String,
    health: u32,
    level: u32,
    experience: u32
}

impl Player {
    fn new_player(name: String) -> Player {
        Player {
            name,
            health: 100,
            level: 0,
            experience: 0
        }
    }

    // immutable borrow of self Player Object
    //Good since all we are doing is printing
    fn print_player(&self) {
        println!("Name: {}", self.name);
        println!("Health: {}", self.health);
        println!("Level: {}", self.level);
        println!("Experience: {}", self.experience);
        println!();
    }

    // Changes to the Player struct must be a reference to a mutable Player object
    fn damage_player(&mut self) {
        self.health -= 1;
    }

    fn heal_player(&mut self) {
        self.health += 1;
    }

    fn give_experience(&mut self) {
        self.experience += 1;
    }
}
fn main() {
    let mut player1 = Player::new_player(String::from("Ivan"));
    player1.print_player();
    player1.damage_player();
    player1.print_player();
    player1.heal_player();
    player1.give_experience();
    player1.print_player();
}