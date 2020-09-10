use crate::{component::Health, component::Component, component::Position, component::Size, entity::Entity};

pub fn get_health(entity: &mut Entity) -> &Health {
    let entity_health = match entity.get_component(|c| if let Component::Health(_) = c { true } else { false }).unwrap() {
        Component::Health(value) => value,
        _ => panic!("No position component found.")
    };

    entity_health
}

pub fn get_position(entity: &mut Entity) -> &Position {
    let entity_position = match entity.get_component(|c| if let Component::Position(_) = c { true } else { false }).unwrap() {
        Component::Position(value) => value,
        _ => panic!("No position component found.")
    };

    entity_position
}

pub fn get_size(entity: &mut Entity) -> &Size {
    let entity_size = match entity.get_component(|c| if let Component::Size(_) = c { true } else { false }).unwrap() {
        Component::Size(value) => value,
        _ => panic!("No Size component found.")
    };

    entity_size
}