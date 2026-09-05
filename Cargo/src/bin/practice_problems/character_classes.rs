enum CharacterClass {
    // default melee damge, some other variable
    Warrior(f32, f32),
    Mage(f32, f32),
    Archer(f32, f32),
}

struct Character {
    name: String,
    health: f32,
    class: CharacterClass,
}

impl Character {
    fn attack(&self) -> f32 {
        match self.class { // matching or switching on the Characters object own class type
            // Ignore second variables only
            CharacterClass::Warrior(warrior_melee_damage, damage_multiplier) => warrior_melee_damage * damage_multiplier,
            CharacterClass::Mage(mage_melee_damage, _) => mage_melee_damage,
            CharacterClass::Archer(archer_melee_damage,_) => archer_melee_damage,
        }
    }

    // Character constructor
    fn make_character(name: String, class: CharacterClass) -> Character {
        Character {
            name,
            health: 100.0,
            class
        }
    }

    // gets the current characters class type as a string
    fn get_class(&self) -> &str {
        match self.class {
            // Ignore both enum variables since we only want to return a string
            CharacterClass::Warrior(_,_) => "Warrior",
            CharacterClass::Mage(_,_) => "Mage",
            CharacterClass::Archer(_,_) => "Archer",
        }
    }

    // get health
    fn get_health(&self) -> f32 {
        self.health
    }
}
fn main() {
    let warrior = Character::make_character(String::from("Ivan"), CharacterClass::Warrior(20.0, 1.5));
    let mage = Character::make_character(String::from("Adi"), CharacterClass::Mage(15.0, 20.0));
    let archer = Character::make_character(String::from("Joeshmoe"), CharacterClass::Archer(5.0, 30.0));


    println!("{} class is: {}, and their attack is: {}, health is: {}", warrior.name, warrior.get_class() ,warrior.attack(), warrior.get_health());
    println!("{} class is: {}, and their attack is: {}, health is: {}", mage.name, mage.get_class(), mage.attack(), mage.get_health());
    println!("{} class is: {}, and their attack is: {}, health is: {}", archer.name, archer.get_class(), archer.attack(), archer.get_health());
}