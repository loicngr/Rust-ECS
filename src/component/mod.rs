// Position Component
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// Size Component
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Size {
    pub height: i32,
    pub width: i32,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Health(pub i32);

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Component {
    Position(Position),
    Size(Size),
    Health(Health),
}

impl Drop for Position {
    fn drop(&mut self) {
        println!("Dropping Position!");
    }
}
