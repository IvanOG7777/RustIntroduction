use std::io:: {self, Write};
use std::collections::HashMap;
use crate::Choice::{AddItem, RemoveItem, UpdateQuantity, FindItem, InventoryValue, UpdatePrice};
use std::collections::hash_map::Entry::{Occupied, Vacant};

struct Item {
    name: String,
    price: Option<f64>,
    quantity: u32,
}

enum Choice {
    AddItem(String, f64),
    RemoveItem(String, u32),
    UpdatePrice(String, f64),
    UpdateQuantity(String, u32),
    FindItem(String),
    InventoryValue(String),
}

enum ChoiceResults <'a> {
    Item(& 'a Item),
    Value(String, f64),
    Ok,
    Err,
    OkNew,
    OkExists
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
        AddItem(item_name, price) => {
            let name = item_name.clone();
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let mut item = entry.into_mut();

                    item.quantity += 1;

                    item.price = Some(price);


                    ChoiceResults::OkExists
                },

                Vacant(new_entry) => {
                    let new_item = Item::create_item(name , Some(0.0));

                    new_entry.insert(new_item);

                    ChoiceResults::OkNew
                }
            }
        },

        RemoveItem(item_name, quantity) => {
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let mut item = entry.into_mut();
                    if quantity >= 1 && quantity < item.quantity {
                        item.quantity -= quantity;
                        ChoiceResults::Ok
                    } else {
                        ChoiceResults::Err
                    }
                },
                Vacant(_) => ChoiceResults::Err
            }
        },

        UpdateQuantity(item_name, new_amount) => {
            match inventory_map.entry(item_name) {
                Occupied(entry) => {

                    if new_amount >= 0 {
                        let item = entry.into_mut();

                        item.quantity = new_amount;
                    }

                    ChoiceResults::Ok
                },

                Vacant(_) => ChoiceResults::Err
            }
        },

        FindItem(item_name) => {
            let name = item_name.clone();
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let item = entry.into_mut();

                    ChoiceResults::Item(item)
                }

                Vacant(_) => ChoiceResults::Err
            }
        },

        InventoryValue(item_name) => {
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let item = entry.into_mut();

                    let result = calculate_value(item);

                    match result {
                        Some(item) => {
                            ChoiceResults::Value(item.0, item.1)
                        }

                        None => ChoiceResults::Err
                    }
                },

                Vacant(_) => ChoiceResults::Err
            }
        },

        UpdatePrice(item_name, new_price) => {
            match inventory_map.entry(item_name) {
                Occupied(entry) => {
                    let item = entry.into_mut();

                    item.price = Some(new_price);

                    ChoiceResults::Ok
                },

                Vacant(_) => {
                    ChoiceResults::Err
                }
            }
        }
    }
}

fn main() {

    let mut item_map: HashMap<String, Item> = HashMap::new();

    'options: loop {
        println!("Please select from following options");
        println!();

        println!("1: Add Item");
        println!("2: Remove Item");
        println!("3: Update Quantity");
        println!("4: Find Item");
        println!("5: Inventory Value");
        println!("6: Update Price");
        println!("7: Quit");

        print!("Select: ");
        io::stdout().flush().unwrap();

        loop {
            let mut user_choice = String::new();

            io::stdin().read_line(&mut user_choice).expect("Failed to read line");

            let option: i32 = user_choice.trim().parse().expect("Not a number");
            println!();
            match option {
                1 => {
                    let mut user_item_name = String::new();
                    let mut user_item_price = String::new();

                    print!("Please enter Item name: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_name).expect("Failed to read line");

                    print!("Please enter Item price: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_price).expect("Failed to read line");


                    let item_name = user_item_name.trim().parse().expect("Failed to trim and parse");
                    let item_price = match user_item_price.trim().parse() {
                        Ok(num) => num,

                        Err(_) => 0.0
                    };


                    let result = handle_choice(AddItem(item_name, item_price), &mut item_map);

                    match result {
                        ChoiceResults::OkExists => {
                            println!("Item: {user_item_name}, exists, added 1 to inventory");
                        },

                        ChoiceResults::OkNew => {
                            println!("Item: {user_item_name}, has been added to inventory");
                        },

                        (_) => {}
                    }
                    break;
                },

                2 => {
                    let mut user_item_name = String::new();
                    let mut user_item_count = String::new();

                    print!("Please enter name of item you want to delete: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_name).expect("Failed to read line");

                    print!("Please enter number of items you want to delete: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_count).expect("Failed to read line");

                    let item_name = user_item_name.trim().parse().expect("Failed to trim and parse");
                    let item_count = match user_item_count.trim().parse() {
                        Ok(num) => num,

                        Err(_) => 0
                    };

                    let result = handle_choice(RemoveItem(item_name, item_count), &mut item_map);

                    match result {
                        ChoiceResults::Ok => {
                            println!("Removed {item_count} of {user_item_name}");
                            println!();
                            break;
                        },

                        ChoiceResults::Err => {
                            println!("Item not found");
                            println!();
                            break;
                        }

                        (_) => {break;}
                    }
                },

                3 => {
                    let mut user_item_name = String::new();
                    let mut user_item_count = String::new();

                    print!("Please enter name of item you want to update: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_name).expect("Failed to read line");

                    print!("Please enter item update count: ");
                    io::stdout().flush().unwrap();

                    let item_name = user_item_name.trim().parse().expect("Failed to trim and parse");
                    let item_count = match user_item_count.trim().parse() {
                        Ok(num) => num,

                        Err(_) => 0,
                    };

                    let result = handle_choice(UpdateQuantity(item_name, item_count), &mut item_map);

                    match result {
                        ChoiceResults::Ok => {
                            println!("Updated: {user_item_name} to: {item_count}");
                            break;
                        },

                        ChoiceResults::Err => {
                            println!("Item not found");
                            break;
                        }

                        (_) => {break;}
                    }
                },

                4 => {
                    let mut user_item_name = String::new();

                    print!("Please enter name of item you want to find: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_name).expect("Failed to read line");

                    let item_name = user_item_name.trim().parse().expect("Failed to trim and parse");

                    let result = handle_choice(FindItem(item_name), &mut item_map);

                    match result {
                        ChoiceResults::Item(item) => {
                            println!("Found item!");
                            println!("Item details");

                            print!("Name: {}", item.name);
                            print!("Quantity: {}", item.quantity);
                            print!("Price per item: {:?}", item.price);

                            break;
                        },

                        ChoiceResults::Err => {
                            println!("Item not found");
                            break;
                        },

                        (_) => {break;}
                    }
                },

                5 => {
                    let mut user_item_name = String::new();

                    print!("Enter name of item to find total inventory value: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_name).expect("Failed to read line");

                    let item_name = user_item_name.trim().parse().expect("Failed to trim and parse");

                    let result = handle_choice(InventoryValue(item_name), &mut item_map);

                    match result {

                        ChoiceResults::Value(name, price) => {
                            println!("Item: {name}, has a total inventory price of: {price}");
                            break;
                        },

                        ChoiceResults::Err => {
                            println!("Item not found");
                            break;
                        },

                        (_) => {break;}
                    }
                },

                6 => {
                    let mut user_item_name = String::new();
                    let mut user_item_price = String::new();

                    print!("Enter name of item to update price: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_name).expect("Failed to read line");

                    print!("Enter new price for item");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut user_item_price).expect("Failed to read line");

                    let item_name = user_item_name.trim().parse().expect("Failed to trim and parse");

                    let item_price = match user_item_price.trim().parse() {
                        Ok(num) => num,

                        Err(_) => {
                            println!("Invalid price");
                            break;
                        },
                    };

                    let result = handle_choice(UpdatePrice(item_name, item_price), &mut item_map);

                    match result {
                        ChoiceResults::Ok => {
                            println!("Price has been updated!");
                            break;
                        }

                        ChoiceResults::Err => {
                            println!("Item not found");
                            break;
                        },
                        (_) => {break;}
                    }
                }

                7 => {
                    println!("Quitting...");
                    break 'options;
                },

                (_) => { println!("Invalid option");println!(); break;}
            }
        }
    }

    for (_name, item) in &item_map {
        println!("Item: {}, price: {:?}, quantity: {}", item.name, item.price, item.quantity);
    }
}