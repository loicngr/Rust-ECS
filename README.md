# My Entity component system

## Create entities factory instance
```rust
    let mut factory_entities = FactoryEntities::new();
```

## Create character entity
```rust
    let character = factory_entities.create_character();
```

## Get character entity position component
```rust
    let character_position = system::get_position(character);
```