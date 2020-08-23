use std::any::Any;
use std::fmt::Debug;

trait Component {}
trait Entity {
    fn add_component<T: 'static + Component>(&mut self, component: T);
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}
impl Component for Position {}

#[derive(Clone)]
struct Inventory {
    slots: Vec<i32>
}
impl Component for Inventory {}


struct Character {
    components: Vec<Box<dyn Component>>
}

struct House {
    components: Vec<Box<dyn Component>>
}

impl Entity for Character {
    fn add_component<T: 'static + Component>(&mut self, component: T) {
        self.components.push(Box::new(component))
    }
}
impl Entity for House {
    fn add_component<T: 'static + Component>(&mut self, component: T) {
        self.components.push(Box::new(component))
    }
}

fn main() {
    
    // Player entity
    let mut entity_character = Character { components: vec![] };

    // House entity
    let mut entity_house = House { components: vec![] };

    // Position Component
    let mut component_position = Position { x: 0, y: 0 };

    // Inventory Component
    let mut component_inventory = Inventory { slots: vec![] };

    entity_character.add_component(component_position.clone());
    entity_character.add_component(component_inventory.clone());

    entity_house.add_component(component_position.clone());

    println!("ok");
}