use std::io;
use rand::Rng;
use crate::Weapon::{Axe, Bow, Sword, IceThrower};
use crate::EnemyType::{Skeleton, Goblin, Dragon};
use crate::Action::{Up, Down, Left, Right, OpenChest, Attack, Retreat, Quit, Yes, No};
const MAX_ENEMIES: usize = 5;

enum Action {
    Up, // w
    Down, // s
    Left, // a
    Right, // d
    OpenChest, // f
    Attack, // e
    Retreat, // r
    Quit, // q
    Yes, // y
    No, // n
}
#[derive(Clone, Copy)]
enum Weapon {
    Sword(f32),
    Bow(f32, u32),
    Axe(f32),
    IceThrower(f32, u32),
}
enum EnemyType {
    Skeleton,
    Goblin,
    Dragon,
}
struct Enemy {
    enemy_type: EnemyType,
    health: f32,
    damage: f32,
}

impl Enemy {
    fn create_enemy(enemy_type: EnemyType) -> Enemy {
        let health:f32 = match enemy_type {
            Skeleton => 100.0,
            Goblin => 75.0,
            Dragon => 150.0,
        };

        let damage: f32 = match enemy_type {
            Skeleton => 10.0,
            Goblin => 2.5,
            Dragon => 20.0,
        };

        Enemy {
            enemy_type,
            health,
            damage
        }
    }

    fn print_enemy_stats(&self) {
        let enemy_type:&str = match self.enemy_type {
            Skeleton => "Skeleton",
            Goblin => "Goblin",
            Dragon => "Dragon",
        };

        println!("Type: {}", enemy_type);
        println!("Health: {}", self.health);
        println!("Damage: {}", self.damage);
    }

    fn attack(&self, player: &mut Player) {
        match self.enemy_type {
            Skeleton => {
                println!("Skeleton attacked {}", player.name);
                player.health -= self.damage;
                if player.health <= 0.0 {
                    println!("Game over you died");
                    return;
                } else {
                    println!("Player health: {}", player.health);
                }
            }

            Goblin => {
                println!("Goblin attacked {}", player.name);
                player.health -= self.damage;
                if player.health <= 0.0 {
                    println!("Game over you died");
                    return;
                } else {
                    println!("Player health: {}", player.health);
                }
            }

            Dragon => {
                println!("Dragon attacked {}", player.name);
                player.health -= self.damage;
                if player.health <= 0.0 {
                    println!("Game over you died");
                    return;
                } else {
                    println!("Player health: {}", player.health);
                }
            }
        }
    }
}
struct Player {
    name: String,
    health: f32,
    weapon: Option<Weapon>,
}

impl Player {
    fn print_player_stats(&self) {
        let weapon_name: &str = match self.weapon {
            Some(Sword(_)) => {"Sword"},
            Some(Bow(_,_)) => {"Bow"},
            Some(Axe(_)) => {"Axe"},
            Some(IceThrower(_,_)) => {"Ice Thrower"},
            None => {"No weapon"},
        };


        println!("Name: {}", self.name);
        println!("Weapon: {}", weapon_name);
        println!("Health: {}" ,self.health);
    }

    fn attack(&self, enemy: &mut Option<Enemy>) {
        match enemy.as_mut().unwrap().enemy_type {
            Skeleton => {
                match self.weapon {
                    Some(Sword(damage)) => {
                        println!("Hit skeleton with Sword for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                    }

                    Some(Bow(damage, mut ammo)) => {
                        if ammo == 0 {
                            println!("Out of ammo no damage");
                            return;
                        }
                        println!("Hit skeleton with Bow for {} damage", damage * 0.15);
                        enemy.as_mut().unwrap().health -= damage * 0.15;
                        ammo -= 1;
                    }

                    Some(Axe(damage)) => {
                        println!("Hit skeleton with Axe for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                    }

                    Some(IceThrower(damage, mut ammo)) => {
                        if ammo == 0 {
                            println!("Out of ammo no damage");
                            return;
                        }
                        println!("Hit skeleton with Ice Thrower for {} damage", damage * 0.25);
                        enemy.as_mut().unwrap().health -= damage * 0.25;
                        ammo -= 1;
                    }

                    None => {
                        print!("Hit skeleton with hand for 1 damage");
                        enemy.as_mut().unwrap().health -= 1.0;
                    }
                }
            },

            Goblin => {
                match self.weapon {
                    Some(Sword(damage)) => {
                        println!("Hit goblin with Sword for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                    }

                    Some(Bow(damage, mut ammo)) => {
                        if ammo == 0 {
                            println!("Out of ammo no damage");
                            return;
                        }
                        println!("Hit goblin with Bow for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                        ammo -= 1;
                    }

                    Some(Axe(damage)) => {
                        println!("Hit goblin with Axe for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                    }

                    Some(IceThrower(damage, mut ammo)) => {
                        if ammo == 0 {
                            println!("Out of ammo no damage");
                            return;
                        }
                        println!("Hit goblin with Ice Thrower for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                        ammo -= 1;
                    }

                    None => {
                        print!("Hit goblin with hand for 1 damage");
                        enemy.as_mut().unwrap().health -= 1.0;
                    }
                }
            },

            Dragon => {
                match self.weapon {
                    Some(Sword(damage)) => {
                        println!("Hit dragon with Sword for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                    }

                    Some(Bow(damage, mut ammo)) => {
                        if ammo == 0 {
                            println!("Out of ammo no damage");
                            return;
                        }
                        println!("Hit dragon with Bow for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                        ammo -= 1;
                    }

                    Some(Axe(damage)) => {
                        println!("Hit dragon with Axe for {} damage", damage);
                        enemy.as_mut().unwrap().health -= damage;
                    }

                    Some(IceThrower(damage, mut ammo)) => {
                        if ammo == 0 {
                            println!("Out of ammo no damage");
                            return;
                        }
                        println!("Hit dragon with Ice Thrower for {} damage", damage * 2.5);
                        enemy.as_mut().unwrap().health -= damage * 2.5;
                        ammo -= 1;
                    }

                    None => {
                        print!("Hit dragon with hand for 1 damage");
                        enemy.as_mut().unwrap().health -= 1.0;
                    }
                }
            }
        }
    }
    fn handle_player_choice(&mut self, action: Option<Action>, game_active: &mut bool, current_location: &mut usize, areas: &[&str], enemy: &mut Option<Enemy>, rand_weapon: Weapon, user_action: &String) {
        match action {
            Some(Up | Left) => {
                if *current_location == 0 {
                    println!("You're already as far back as you can go.");
                } else {
                    *current_location -= 1;
                    println!("You move to: {}", areas[*current_location]);
                }
            }

            Some(Down | Right) => {
                if *current_location == areas.len() - 1 {
                    println!("You're already as far forward as you can go.");
                } else {
                    *current_location += 1;
                    println!("You move to: {}", areas[*current_location]);
                }
            }

            Some(OpenChest) => {
                println!("You open a chest, change weapon?");
                if let Some(response) = read_action(&user_action) {
                    match response {
                        Yes => {
                            self.weapon = Some(rand_weapon);
                            println!("You equip the new weapon.");
                        }
                        No => {
                            println!("You leave it behind.");
                        }
                        _ => {
                            println!("Not a yes/no answer — leaving it behind.");
                        }
                    }
                }
            }

            Some(Attack) => {
                self.attack(enemy);
            }

            Some(Retreat) => {
                if *current_location == 0 {
                    *current_location += 1;
                }
                if *current_location == areas.len() {
                    *current_location -= 1;
                }
            }

            Some(Quit) => {
                println!("Quitting game");
                *game_active = false;
            },

            _ => {
                println!("That action doesn't apply right now.");
            }
        }
    }
}

fn read_action(action: &String) -> Option<Action> {
    match action.trim() {
        "w" => Some(Up),
        "s" => Some(Down),
        "a" => Some(Left),
        "d" => Some(Right),
        "f" => Some(OpenChest),
        "e" => Some(Attack),
        "r" => Some(Retreat),
        "q" => Some(Quit),
        "y" => Some(Yes),
        "n" => Some(No),
        _ => None,
    }
}

fn random_weapon() -> Weapon {
    match rand::thread_rng().gen_range(0..4) {
        0 => Sword(32.0),
        1 => Bow(15.0, 25),
        2 => Axe(50.0),
        _ => IceThrower(20.0, 15),
    }
}

fn main() {

    // Create array of enemies initially set to None (empty).
    let mut enemy_array: [Option<Enemy>; MAX_ENEMIES] = [const { None }; MAX_ENEMIES];
    let locations = ["Forest", "Broken City", "Underground cellar", "Torn down school", "Garden of Eden"];
    let mut game_active = true;
    let mut current_location = 0;

    // Fill in enemy_array with rand generated enemies
    for i in 0..MAX_ENEMIES {
        let rand = rand::thread_rng().gen_range(0..3);
        let new_enemy: Enemy = match rand {
            0 => Enemy::create_enemy(Skeleton),
            1 => Enemy::create_enemy(Goblin),
            2 => Enemy::create_enemy(Dragon),
            _ => panic!("Failed to create an enemy"),
        };
        enemy_array[i] = Some(new_enemy);
    }

    println!("Welcome come to my game");
    println!("After making character");
    println!("Press Q to end game");
    println!();
    println!();


    let mut user_name = String::new();
    let mut user_weapon = String::new();
    println!("Make your character!");
    println!("Choose your name!");
    io::stdin().read_line(&mut user_name).expect("Failed to read");
    println!("Choose your weapon!");
    println!("1: Sword");
    println!("2: Bow");
    println!("3: Axe");
    println!("4: Ice Thrower");

    io::stdin().read_line(&mut user_weapon).expect("Failed to read");

    let weapon_number: i32 = user_weapon.trim().parse().expect("Not a number");
    user_name = String::from(user_name.trim());

    let mut player = Player {
        name: user_name,
        health: 100.0,
        weapon: match weapon_number {
            1 => Some(Sword(32.0)),
            2 => Some(Bow(15.0, 25)),
            3 => Some(Axe(50.0)),
            4 => Some(IceThrower(20.0, 15)),
            _ => None
        }
    };

    println!("You are: ");
    player.print_player_stats();
    println!();

    while game_active {
        let mut user_action = String::new();
        let rand_enemy_index = rand::thread_rng().gen_range(0..locations.len());

        let enemy = &mut enemy_array[rand_enemy_index];

        while rand_enemy_index == current_location {
            let does_enemy_attack = rand::thread_rng().gen_range(0..=1);
            println!("An enemy has appeared!");
            println!("Do you attack or retreat?");
            println!("e: Attack");
            println!("r: Retreat");
            user_action.clear();
            io::stdin().read_line(&mut user_action).expect("Failed to read line");
            user_action = String::from(user_action.trim().to_lowercase());

            let action = read_action(&user_action);

            player.handle_player_choice(action, &mut game_active, &mut current_location, &locations, enemy, player.weapon.unwrap(), &user_action);
            enemy.as_ref().unwrap().print_enemy_stats();

            if does_enemy_attack == 1 {
                enemy.as_ref().unwrap().attack(&mut player);
                println!("Enemy Attacked!");
                player.print_player_stats();
            }
        }

        println!("You are in: {}", locations[current_location]);
        println!("Where would you like to go");
        println!("w: Up");
        println!("s: Down");
        println!("a: Left");
        println!("d: Right");

        user_action.clear();
        io::stdin().read_line(&mut user_action).expect("Failed to read line");
        user_action = String::from(user_action.trim().to_lowercase());


        let action = read_action(&user_action);

        player.handle_player_choice(action, &mut game_active, &mut current_location, &locations, enemy, player.weapon.unwrap(), &user_action);
        println!();
    }
}