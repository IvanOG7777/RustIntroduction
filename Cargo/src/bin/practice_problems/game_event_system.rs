use std::fmt::format;
use rand::Rng;
use crate::CharacterClass::{Archer, Mage, Warrior}; // lets us use enum names directly instead of saying CharacterClass::Warrior or CharacterClass::Archer
use crate::GameEvent::{PlayerJoined, PlayerMoved, PlayerAttacked, PLayerDied, GameEnded};

const MAX_HEALTH: f32 = 100.0;
const MAX_PLAYERS: usize = 5;

enum GameEvent{
    PlayerJoined(String, i32),
    PlayerMoved(String, i32, (i32, i32, i32)),
    PlayerAttacked,
    PLayerDied(bool),
    GameEnded([Player; MAX_PLAYERS]),
}

enum CharacterClass {
    // default melee damge, some other variable, max health
    Warrior(f32, f32, f32),
    Mage(f32, f32, f32),
    Archer(f32, f32, f32),
}

struct Character {
    name: String, // name of the character class as string
    class: CharacterClass,
}

impl Character {
    fn create_character(chosen_character: String) -> Character {
        // get class type from passed name
        // used as single source of truth when creating a character class
        let class_type = match chosen_character.as_str() {
            "Warrior" => Warrior(20.0, 1.5, MAX_HEALTH),
            "Mage" => Mage(15.0, 20.0, MAX_HEALTH),
            "Archer" => Archer(5.0, 30.0, MAX_HEALTH),
            _ => panic!("Invalid character class: {}", chosen_character),
        };

        Character {
            name: chosen_character,
            class: class_type,
        }
    }

    fn get_attack_damage(&self) -> f32 {
        match self.class {
            Warrior(melee_damage,damage_multiplier, _) => melee_damage * damage_multiplier,
            Mage(melee_damage, _, _) => melee_damage,
            Archer(melee_damage,_, _) => melee_damage
        }
    }

    fn get_heath(&self) -> f32 {
        match self.class {
            Warrior(_, _, health) => health,
            Mage(_, _, health) => health,
            Archer(_, _, health) => health,
        }
    }
}

struct Player {
    username: String,
    id: i32,
    position: (i32, i32, i32),
    character: Character,
    is_alive: bool
}

impl Player {
    fn create_player(username: String, chosen_character: String) -> Player {
        Player {
            username,
            id: rand::thread_rng().gen_range(1..=100),
            position: (0,0,0),
            is_alive: true,
            character: Character::create_character(chosen_character),
        }
    }

    fn print_player_stats(&self) {
        println!("Username: {}", self.username);
        println!("ID: {}", self.id);
        println!("Position: ({}, {}, {})", self.position.0, self.position.1, self.position.2);
        println!("Character: {}", self.character.name);
        println!("Health: {}", self.character.get_heath());
        println!("Melee Damage: {}", self.character.get_attack_damage());
    }
}

// fn handle_event(GameEvent)

fn main() {
    let player1 = Player::create_player(String::from("Ivan"), String::from("Warrior"));
    let player2 = Player::create_player(String::from("Cricri"), String::from("Mage"));
    let player3 = Player::create_player(String::from("Marvin"), String::from("Archer"));
    let player4 = Player::create_player(String::from("Ammaar"), String::from("Warrior"));
    let player5 = Player::create_player(String::from("Mike"), String::from("Archer"));


    let players: [Player; MAX_PLAYERS] = [player1, player2, player3, player4, player5];

    for player in  players {
        player.print_player_stats();
        println!();
    }
}