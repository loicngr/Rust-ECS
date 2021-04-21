use crate::component::{Component, Health, Position, Size};

use super::{store::EntityStore, Entity};
use _const::HEALTH;

pub struct FactoryEntities {
    pub es: EntityStore,
}

#[allow(dead_code)]
impl FactoryEntities {
    pub fn new() -> Self {
        let es = EntityStore::new();
        FactoryEntities { es }
    }

    pub fn create_character(&mut self) -> &mut Entity {
        // Make entity
        let entity = self
            .es
            .create_entity(String::from("player"))
            .with_component(Component::Position(Position { x: 0, y: 0 }))
            .with_component(Component::Health(Health(HEALTH)))
            .with_component(Component::Size(Size {
                height: 10,
                width: 10,
            }))
            .end();

        entity
    }

    pub fn get_entity_by_name(&mut self, name: String) -> Option<&mut Entity> {
        self.es
            .entities
            .iter_mut()
            .find(|c| if c.name == name { true } else { false })
    }

    pub fn get_entity_by_id(&mut self, id: usize) -> Option<&mut Entity> {
        self.es
            .entities
            .iter_mut()
            .find(|c| if c.id == id { true } else { false })
    }

    pub fn create_enemy(&mut self) -> &mut Entity {
        // Make entity
        let entity = self
            .es
            .create_entity(String::from("enemy"))
            .with_component(Component::Position(Position { x: 0, y: 0 }))
            .with_component(Component::Health(Health(HEALTH)))
            .with_component(Component::Size(Size {
                height: 10,
                width: 10,
            }))
            .end();

        entity
    }
}
