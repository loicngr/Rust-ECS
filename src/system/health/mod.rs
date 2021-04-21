use _const::DAMAGE_RANGE;
use component::{Component, Health};
use entity::Entity;

pub fn hit_once(health: &mut Health, damage_taken: Option<i32>) {
    health.0 -= match &damage_taken {
        None => DAMAGE_RANGE,
        Some(v) => v.clone(),
    };
}

pub fn get(entity: &Entity) -> &Health {
    match entity
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
        _ => panic!("No health component found."),
    }
}

pub fn get_mutable(entity: &mut Entity) -> &mut Health {
    match entity
        .get_component_mutable(|c| {
            if let Component::Health(_) = c {
                true
            } else {
                false
            }
        })
        .unwrap()
    {
        Component::Health(value) => value,
        _ => panic!("No health component found."),
    }
}
