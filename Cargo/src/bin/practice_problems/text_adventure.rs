use std::io;
use crate::Weapon::{Axe, Bow, Sword};

enum Action {
    Up,
    Down,
    Left,
    Right,
    OpenChest,
    Attack,
    Quit,
}

enum Weapon {
    Sword(f32),
    Bow(f32, u32),
    Axe(f32)
}

enum Enemy {
    Skeleton,
    Goblin,
    Dragon,
}
struct Player {
    name: String,
    health: f32,
    weapon: Option<Weapon>,
}
fn main() {

    let mut user_name = String::new();
    let mut user_weapon = String::new();
    println!("Make your character!");
    println!("Choose your name!");
    io::stdin().read_line(&mut user_name).expect("Failed to read");
    println!("Choose your weapon!");
    println!("1: Sword");
    println!("2: Bow");
    println!("3: Axe");

    io::stdin().read_line(&mut user_weapon).expect("Failed to read");

    let weapon_number: i32 = user_weapon.trim().parse().expect("Not a number");

    let player = Player {
        name: user_name,
        health: 100.0,
        weapon: match weapon_number {
            1 => Some(Sword(32.0)),
            2 => Some(Bow(15.0, 25)),
            3 => Some(Axe(50.0)),
            _ => None
        }
    };


}