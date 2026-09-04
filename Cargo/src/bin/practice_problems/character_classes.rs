enum CharacterClass {
    Warrior,
    Mage,
    Archer,
}

struct Character {
    name: String,
    health: u32,
    class: CharacterClass,
}

impl Character {
    fn attack(&self) -> u32 {
        match self.class { // matching or switching on the Characters object own clas type
            CharacterClass::Warrior => 20,
            CharacterClass::Mage => 15,
            CharacterClass::Archer => 10,
        }
    }

    fn make_character(name: String, class: CharacterClass) -> Character {
        Character {
            name,
            health: 100,
            class
        }
    }
}
fn main() {
    let warrior = Character::make_character(String::from("Ivan"), CharacterClass::Warrior);
    let mage = Character::make_character(String::from("Adi"), CharacterClass::Mage);
    let archer = Character::make_character(String::from("Joeshmoe"), CharacterClass::Archer);


    println!("{} class is {}, and their attack is: {}", warrior.name, warrior.class, warrior.attack());
    println!("{} class is {}, and their attack is: {}", mage.name, mage.class, mage.attack());
    println!("{} class is {}, and their attack is: {}", archer.name, archer.class, archer.attack());
}