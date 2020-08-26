enum ComponentEnum {
    Position,
    Size
}

trait Component {}

// Position Component
#[derive(PartialEq, PartialOrd, Debug)]
struct Position {
    x: i32,
    y: i32
}

// Size Component
#[derive(PartialEq, PartialOrd, Debug)]
struct Size {
    height: i32,
    width: i32
}

impl Component for Position {}
impl Component for Size {}

struct Entity {
    id: usize,
    components: Vec<Box<dyn Component>>
}

impl Entity {
    fn new(index: usize) -> Self {
        Entity { id: index, components: vec![] }
    }

    // Add a component in Entity
    fn add_component<T: 'static + Component>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }
}

struct EntityStore {
    entities: Vec<Entity>,
    current_index: usize,
}
impl EntityStore {
    fn new() -> EntityStore {
        EntityStore { entities: vec![], current_index: 0 }
    }

    fn generate_index(&self) -> usize {
        unimplemented!();
    }

    // Stop creation system and update EntityStore current_index
    fn end(&mut self) -> &mut Entity {
        let entity = self.entities.get_mut(self.current_index).unwrap();
        self.current_index = self.current_index + 1;
        entity
    }

    fn create_entity(&mut self) -> &mut Self {
        let mut entity = Entity::new(self.current_index);
        self.entities.push(entity);

        self
    }

    // Add component to entity
    fn with_component<T: 'static + Component>(&mut self, component: T) ->  &mut Self {
        let mut entity = self.entities.get_mut(self.current_index).unwrap();
        entity.add_component(component);

        self
    }
}

fn main() {
    let mut es = EntityStore::new();

    // Make entity
    let mut entity1 = es
        .create_entity()
        .with_component(Position { x: 0, y: 0 })
        .with_component(Size { height: 10, width: 10 })
        .end();

    // Get entity position component
    // let component_position_entity1 = entity1.get_component(ComponentEnum::Position);
}