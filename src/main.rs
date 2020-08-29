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

#[derive(PartialEq, PartialOrd, Debug)]
enum Component {
    Position(Position),
    Size(Size)
}

#[derive(Debug)]
struct Entity {
    id: usize,
    components: Vec<Component>
}

impl Entity {
    fn new(index: usize) -> Self {
        Entity { id: index, components: vec![] }
    }

    // Add a component in Entity
    fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    fn get_component(
        &self,
        predicate: impl Fn(&&Component) -> bool
    ) -> Option<&Component> {
        self.components.iter().find(predicate)
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

    // Stop creation system and update EntityStore current_index
    fn end(&mut self) -> &mut Entity {
        let entity = self.entities.get_mut(self.current_index).unwrap();
        self.current_index = self.current_index + 1;
        entity
    }

    fn create_entity(&mut self) -> &mut Self {
        let entity = Entity::new(self.current_index);
        self.entities.push(entity);

        self
    }

    // Add component to entity
    fn with_component(&mut self, component: Component) ->  &mut Self {
        let entity = self.entities.get_mut(self.current_index).unwrap();
        entity.add_component(component);

        self
    }
}

struct FactoryEntities {
    es: EntityStore
}
impl FactoryEntities {
    fn new() -> Self {
        let es = EntityStore::new();
        FactoryEntities { es }
    }

    fn create_character(&mut self) -> &mut Entity {
        // Make entity
        let entity = self.es
            .create_entity()
            .with_component(Component::Position(Position { x: 0, y: 0 }))
            .with_component(Component::Size(Size { height: 10, width: 10 }))
            .end();

        entity
    }
}


fn main() {
    let mut factory_entities = FactoryEntities::new();

    let character = factory_entities.create_character();

    let character_position = match character.get_component(|c| if let Component::Position(_) = c { true } else { false }).unwrap() {
        Component::Position(value) => value,
        _ => panic!("No position component found.")
    };

    let character_size = match character.get_component(|c| if let Component::Size(_) = c { true } else { false }).unwrap() {
        Component::Size(value) => value,
        _ => panic!("No Size component found.")
    };

    println!("{:?}", character_position);
    println!("{:?}", character_size);
}