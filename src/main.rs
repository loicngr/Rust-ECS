use component::Position;
use entity::factory::FactoryEntities;

mod component;
mod entity;
mod system;

fn main() {
    let mut factory_entities = FactoryEntities::new();

    let character = factory_entities.create_character();
    let character_position = system::get_position(character);
    system::print_position(&character_position);

    let mut_character_position: &mut Position = system::get_position_mutable(character);
    let new_character_position = Position { x: 6, y: 66 };
    system::move_position(mut_character_position, &new_character_position);

    let character_position = system::get_position(character);
    system::print_position(&character_position);
}
