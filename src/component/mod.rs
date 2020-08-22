pub struct Component<T> (pub T);

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Slot {
    pub id: u32,
    pub name: &'static str,
    pub order: u32
}
pub struct Inventory {
    pub slots: Vec<Slot>
}