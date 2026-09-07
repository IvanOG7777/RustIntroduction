use crate::AttackType::{Physical, Fire, Ice, Lighting};
use crate::EnemyType::{Human, Dragon, IceMonster, Robot};

enum AttackType {
    Physical,
    Fire,
    Ice,
    Lighting
}

enum EnemyType {
    Human,
    Dragon,
    IceMonster,
    Robot,
}

fn calculate_damage(enemy_type: EnemyType, attack_type: AttackType) {
    match attack_type {
        Physical => {
            match enemy_type {
                Human => {
                    println!("Physical + Human -> High damage");
                }

                Dragon => {
                    println!("Physical + Dragon -> Low damage");
                }

                IceMonster => {
                    println!("Physical + Ice Monster -> Medium damage");
                }

                Robot => {
                    println!("Physical + Robot -> low damage");
                }
            }
        }

        Fire => {
            match enemy_type {
                Human => {
                    println!("Fire + Human -> High damage");
                }

                Dragon => {
                    println!("Fire + Dragon -> Low damage");
                }

                IceMonster => {
                    println!("Fire + Ice Monster -> High damage");
                }

                Robot => {
                    println!("Fire + Robot -> Medium damage");
                }
            }
        }

        Ice => {
            match enemy_type {
                Human => {
                    println!("Ice + Human -> Medium damage");
                }

                Dragon => {
                    println!("Ice + Dragon -> High damage");
                }

                IceMonster => {
                    println!("Ice + Ice Monster -> Low damage");
                }

                Robot => {
                    println!("Ice + Robot -> Medium damage");
                }
            }
        }

        Lighting => {
            match enemy_type {
                Human => {
                    println!("Lighting + Human -> Medium damage");
                }

                Dragon => {
                    println!("Lighting + Dragon -> Medium damage");
                }

                IceMonster => {
                    println!("Lighting + Ice Monster -> Low damage");
                }

                Robot => {
                    println!("Lighting + Robot -> High damage");
                }
            }
        }
    }
}

fn main() {

    calculate_damage(Human, Lighting);

    calculate_damage(Dragon, Fire);

    calculate_damage(IceMonster, Lighting);

    calculate_damage(Robot, Fire);
}