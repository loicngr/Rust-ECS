use crate::component::Component;

use super::Entity;

pub struct EntityStore {
    pub entities: Vec<Entity>,
    pub current_index: usize,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            entities: vec![],
            current_index: 0,
        }
    }

    // Stop creation system and update EntityStore current_index
    pub fn end(&mut self) -> &mut Entity {
        let entity = self.entities.get_mut(self.current_index).unwrap();
        self.current_index = self.current_index + 1;
        entity
    }

    pub fn create_entity(&mut self, name: String) -> &mut Self {
        let entity = Entity::new(self.current_index, name);
        self.entities.push(entity);

        self
    }

    // Add component to entity
    pub fn with_component(&mut self, component: Component) -> &mut Self {
        let entity = self.entities.get_mut(self.current_index).unwrap();
        entity.add_component(component);

        self
    }
}
