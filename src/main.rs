use component::{Component, Position};
use entity::factory::FactoryEntities;

mod _const;
mod component;
mod entity;
mod system;

fn main() {
    let mut factory_entities = FactoryEntities::new();

    let character = factory_entities.create_character();
    let character_position = system::position::get(character);

    system::print(&Component::Position(character_position.clone()));

    let mut_character_position: &mut Position = system::position::get_mutable(character);
    let new_character_position = Position { x: 6, y: 66 };
    system::position::move_vec(mut_character_position, &new_character_position);
    system::position::move_frontward(mut_character_position, None);
    system::position::move_left(mut_character_position, None);

    let character_position = system::position::get(character);
    system::print(&Component::Position(character_position.clone()));

    let mut_character_health = system::health::get_mutable(character);
    system::health::hit_once(mut_character_health, None);
    let character_health = system::health::get(character);
    system::print(&Component::Health(character_health.clone()));
}
