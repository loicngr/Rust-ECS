pub mod store;
pub mod factory;

use crate::{component::Component};

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub name: String,
    pub components: Vec<Component>
}

impl Entity {
    pub fn new(index: usize, name: String) -> Self {
        Entity { id: index, name, components: vec![] }
    }

    // Add a component in Entity
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn get_component(
        &self,
        predicate: impl Fn(&&Component) -> bool
    ) -> Option<&Component> {
        self.components.iter().find(predicate)
    }
}