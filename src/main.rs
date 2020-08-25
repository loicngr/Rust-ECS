use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::fmt::{Pointer, Debug};
use std::path::Display;
use std::fmt;
use std::io::Write;
use std::borrow::{Borrow, BorrowMut};

enum EntityEnum {
    Character
}


enum ComponentEnum {
    Position
}

trait ComponentTrait {}
trait EntityTrait {
    fn len(&self) -> usize;
    fn id(&self) -> usize;
    fn components(&self) -> &Vec<Box<dyn ComponentTrait>>;
    fn add_component(&mut self, component_enum: ComponentEnum);
}

struct PositionComponent {
    x: i32,
    y: i32
}

impl PositionComponent {
    fn new(x: i32, y: i32) -> Self {
        PositionComponent { x, y }
    }
}

impl ComponentTrait for PositionComponent {}

struct CharacterEntity {
    id: usize,
    components: Vec<Box<dyn ComponentTrait>>,
}

impl CharacterEntity {
    fn new() -> CharacterEntity {
        CharacterEntity { id: 0, components: vec![] }
    }
}
impl EntityTrait for CharacterEntity {
    fn len(&self) -> usize {
        self.components.len()
    }
    fn id(&self) -> usize {
        self.id
    }
    fn components(&self) -> &Vec<Box<dyn ComponentTrait>> {
        self.components.as_ref()
    }
    fn add_component(&mut self, component_enum: ComponentEnum) {
        let component = match component_enum {
            ComponentEnum::Position => PositionComponent::new(0, 0)
        };

        self.components.push(Box::new(component));
    }
}

impl Debug for dyn EntityTrait {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Entity Components {{{}}}", self.len());
        write!(f, ", ID {{{}}}", self.id())
    }
}

struct EntityStore {
    entities: Vec<Box<dyn EntityTrait>>,
    current_index: usize
}

impl EntityStore {
    fn new() -> Self {
        EntityStore { entities: vec![], current_index: 0 }
    }

    fn es_end(&mut self) {
        self.current_index = self.current_index + 1;
    }

    fn create_entity(&mut self, entity_enum: EntityEnum) -> &mut Self {
        let mut entity = match entity_enum {
            EntityEnum::Character => CharacterEntity::new(),
        };
        entity.id = self.current_index.clone();

        self.entities.push(Box::new(entity));
        self
    }

    fn get_entity(&mut self, index: usize) -> &mut Box<dyn EntityTrait> {
        self.entities.get_mut(index).unwrap()
    }

    fn with_component(&mut self, component: ComponentEnum) -> &mut Self {
        let mut entity = self.get_entity(self.current_index);
        entity.add_component(component);
        println!("{:?}", entity);
        self
    }
}

fn main() {
    let mut es = EntityStore::new();

    es
        .create_entity(EntityEnum::Character)
        .with_component(ComponentEnum::Position)
        .es_end();

    println!("main");
}