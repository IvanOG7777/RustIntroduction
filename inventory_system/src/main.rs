use std::collections::HashMap;
use std::path::Prefix;

struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

enum Choice {
    AddItem(Item),
    RemoveItem(Item, u32),
    UpdateQuantity(Item, u32),
    FindItem(String),
    InventoryValue,
}

impl Item {
    fn create_item(name: String, price: f64) -> Item {
        Item {
            name,
            price,
            quantity: 1
        }
    }
}

fn handle_choice(choice: Choice, inventory_map: HashMap<&String, Item>) -> Option<Item> {

}

fn main() {

    let item_map: HashMap<String, Item> = HashMap::new();

    
}