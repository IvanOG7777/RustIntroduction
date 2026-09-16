use std::collections::HashMap;
use crate::Choice::{AddItem, RemoveItem, UpdateQuantity, FindItem, InventoryValue};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::result;

struct Item {
    name: String,
    price: Option<f64>,
    quantity: u32,
}

enum Choice {
    AddItem(String),
    RemoveItem(String, u32),
    UpdateQuantity(String, u32),
    FindItem(String),
    InventoryValue(String),
}

enum ChoiceResults <'a> {
    Item(& 'a Item),
    Value(String, f64),
    None,
}

impl Item {
    fn create_item(name: String, price: Option<f64>) -> Item {
        Item {
            name,
            price,
            quantity: 1
        }
    }
}

fn calculate_value(item: &mut Item) -> Option<(String, f64)> {
    let item_name = item.name.clone();

    let total_value = match item.price {
        Some(value) => {
            (item.quantity as f64) * value
        },

        None => 0.0
    };

    Some((item_name, total_value))
}

fn handle_choice(choice: Choice, mut inventory_map: &mut HashMap<String, Item>) -> ChoiceResults {
    match choice {
        AddItem(item_name) => {
            let name = item_name.clone();
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let mut item = entry.into_mut();

                    item.quantity += 1;

                    ChoiceResults::None
                },

                Vacant(new_entry) => {
                    let new_item = Item::create_item(name , None);

                    new_entry.insert(new_item);

                    ChoiceResults::None
                }
            }
        },

        RemoveItem(item_name, quantity) => {
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let mut item = entry.into_mut();
                    if quantity >= 1 && quantity < item.quantity {
                        item.quantity -= quantity;
                        ChoiceResults::None
                    } else {
                        ChoiceResults::None
                    }
                },
                Vacant(_) => ChoiceResults::None
            }
        },

        UpdateQuantity(item_name, new_amount) => {
            match inventory_map.entry(item_name) {
                Occupied(entry) => {

                    if new_amount >= 0 {
                        let item = entry.into_mut();

                        item.quantity = new_amount;
                    }

                    ChoiceResults::None
                },

                Vacant(_) => ChoiceResults::None
            }
        },

        FindItem(item_name) => {
            let name = item_name.clone();
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let item = entry.into_mut();

                    ChoiceResults::Item(item)
                }

                Vacant(_) => ChoiceResults::None
            }
        },

        InventoryValue(item_name) => {
            let name = item_name.clone();
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let item = entry.into_mut();

                    let result = calculate_value(item);

                    match result {
                        Some(item) => {
                            ChoiceResults::Value(item.0, item.1)
                        }

                        None => ChoiceResults::None
                    }
                },

                Vacant(_) => ChoiceResults::None
            }
        }
    }
}

fn main() {

    let item_map: HashMap<String, Item> = HashMap::new();


}