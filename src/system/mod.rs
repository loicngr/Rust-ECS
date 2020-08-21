use Position;

pub fn move_position(mut position: &mut Position, direction: (i32, i32)) {
    let (direction_x, direction_y) = direction;

    position.x += direction_x;
    position.y += direction_y;
}

pub fn equal_positions(position1: &Position, position2: &Position) -> bool {
    if position1.x == position2.x && position1.y == position2.y {
        return true;
    }
    false
}