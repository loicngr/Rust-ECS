use component::Component;

pub struct Character<T> {
    pub components: Vec<Component::<T>>,
}