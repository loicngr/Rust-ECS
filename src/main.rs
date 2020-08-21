use entity::Character;
use component::Position;

mod component;
mod entity;
mod system;

fn main() {
    let mut e_character = Character { position: Position { x: 0, y: 0 } };
    println!("{:?}", e_character.position);

    system::move_position(&mut e_character.position, (1, 2));

    println!("{:?}", e_character.position);
}