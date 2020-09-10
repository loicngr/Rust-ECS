mod system;
mod component;
mod entity;

use entity::factory::FactoryEntities;

fn main() {
    let mut factory_entities = FactoryEntities::new();

    let character = factory_entities.create_character();
    let character_position = system::get_position(character);

    println!("{:?}", character_position);

    let enemy_01 = factory_entities.create_enemy();

    println!("{:?}", enemy_01);
}