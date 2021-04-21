pub mod health;
pub mod position;

use crate::{component::Component, component::Health, component::Size, entity::Entity};

#[allow(dead_code)]
pub fn get_health(entity: &mut Entity) -> &Health {
    let entity_health = match entity
        .get_component(|c| {
            if let Component::Health(_) = c {
                true
            } else {
                false
            }
        })
        .unwrap()
    {
        Component::Health(value) => value,
        _ => panic!("No position component found."),
    };

    entity_health
}

#[allow(dead_code)]
pub fn get_size(entity: &mut Entity) -> &Size {
    let entity_size = match entity
        .get_component(|c| {
            if let Component::Size(_) = c {
                true
            } else {
                false
            }
        })
        .unwrap()
    {
        Component::Size(value) => value,
        _ => panic!("No Size component found."),
    };

    entity_size
}

pub fn print(c: &Component) {
    println!("{:#?}", c);
}
