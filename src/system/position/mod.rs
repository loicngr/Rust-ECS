use _const::{MOVE_BACK_RANGE, MOVE_FRONT_RANGE, MOVE_LEFT_RANGE, MOVE_RIGHT_RANGE};
use component::{Component, Position};
use entity::Entity;

pub fn move_vec(position: &mut Position, new_position: &Position) {
    position.x = new_position.x;
    position.y = new_position.y;
}

pub fn move_frontward(position: &mut Position, move_range: Option<i32>) {
    position.x += match &move_range {
        None => MOVE_FRONT_RANGE,
        Some(v) => v.clone(),
    };
}

#[allow(dead_code)]
pub fn move_backward(position: &mut Position, move_range: Option<i32>) {
    position.x -= match &move_range {
        None => MOVE_BACK_RANGE,
        Some(v) => v.clone(),
    };
}

#[allow(dead_code)]
pub fn move_left(position: &mut Position, move_range: Option<i32>) {
    position.y -= match &move_range {
        None => MOVE_LEFT_RANGE,
        Some(v) => v.clone(),
    };
}

#[allow(dead_code)]
pub fn move_right(position: &mut Position, move_range: Option<i32>) {
    position.y += match &move_range {
        None => MOVE_RIGHT_RANGE,
        Some(v) => v.clone(),
    };
}

pub fn get_mutable(entity: &mut Entity) -> &mut Position {
    let entity_position = match entity
        .get_component_mutable(|c| {
            if let Component::Position(_) = c {
                true
            } else {
                false
            }
        })
        .unwrap()
    {
        Component::Position(value) => value,
        _ => panic!("No position component found."),
    };

    entity_position
}

pub fn get(entity: &Entity) -> &Position {
    match entity
        .get_component(|c| {
            if let Component::Position(_) = c {
                true
            } else {
                false
            }
        })
        .unwrap()
    {
        Component::Position(value) => value,
        _ => panic!("No position component found."),
    }
}
