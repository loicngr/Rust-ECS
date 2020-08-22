use entity::Character;
use component::{Position, Component, Inventory, Slot};

mod component;
mod entity;
mod system;

fn main() {
    let mut e_character = Character { components: vec![] }; // Entité character
    
    let mut c_position = Position { x: 10, y: 50 }; // Composant de position
    
    e_character.components.push(Component::<Position>(c_position) ); // Ajout du composant de position dans l'entité character

    let mut c_inventory = Inventory { slots: vec![] };
    c_inventory.slots.push(Slot { id: 1, name: "Book", order: 1 });

    e_character.components.push(Component::<Inventory>(c_inventory));
}