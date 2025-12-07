enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};

type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

fn main() {
    let inventory: PlayerInventory = RwLock::new(vec![]);
    println!("Inventory: {:?}", inventory);
}