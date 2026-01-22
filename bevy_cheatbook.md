# Bevy Query 101

## What is an Entity?

In Bevy's **Entity Component System (ECS)**, an **Entity** is simply a **unique identifier** for an object in your game world.

Think of it like this:
- **Entity** = A unique ID number (like a social security number for game objects)
- **Component** = Data attached to that entity (health, position, sprite)
- **System** = Logic that operates on entities with specific components

### Example:
```rust
// When you spawn this:
commands.spawn((
    Player { health: 3 },
    Transform::from_xyz(0.0, 0.0, 0.0),
    Sprite { ... },
));

// Bevy creates an Entity with a unique ID and attaches all those components to it
// The Entity itself doesn't hold data - it just points to the components
```

## Query Types Explained

### 1. `Query<&SomeComponent>` - Get component data
```rust
fn example(query: Query<&Player>) {
    // Get the Player component data from all entities that have Player
    for player in &query {
        println!("Player health: {}", player.health);
    }
}
```
**Use when:** You just want to read/write the component data, not the entity ID.

---

### 2. `Query<Entity, With<SomeComponent>>` - Get entity IDs
```rust
fn example(query: Query<Entity, With<Player>>) {
    // Get the Entity IDs of all entities that have Player component
    for entity in &query {
        println!("Entity ID: {:?}", entity);
        // You can despawn or reference these entities
    }
}
```
**Use when:** You need to despawn, pass Entity to other systems, or look up entities.

---

### 3. `Query<(Entity, &SomeComponent)>` - Get both
```rust
fn example(query: Query<(Entity, &Player)>) {
    // Get both the Entity ID and the Player component data
    for (entity, player) in &query {
        println!("Entity {:?} has health: {}", entity, player.health);
        // You can use the entity to despawn it later
        if player.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
```
**Use when:** You need both the entity ID (for despawning/referencing) AND the component data.

---

### 4. `Query<(), With<SomeComponent>>` - Just check existence
```rust
fn example(query: Query<(), With<Player>>) {
    // Count how many entities have the Player component
    let count = query.iter().count();
    println!("Number of players: {}", count);
}
```
**Use when:** You only need to know how many entities have a component, not their data.

---

### 5. `Single<SomeComponent>` - Exactly one entity
```rust
fn example(query: Single<&Player>) {
    // This will panic if there's not exactly one entity with Player
    let player = query.into_inner();
    println!("Player health: {}", player.health);
}
```
**Use when:** You know there's exactly one entity (like the player).

---

## Real Examples from Hotline Miami Clone

### ❌ Inefficient Query (Don't Do This):
```rust
// Getting Entity but only using .count()
text_screen_query: Query<(Entity, &TextScreen)>,
// ...
if text_screen_query.count() == 0 { ... }
```
**Problem:** You're fetching Entity IDs you don't need.

### ✅ Better Version:
```rust
// Just get the component
text_screen_query: Query<&TextScreen>,
// OR just check existence
text_screen_query: Query<(), With<TextScreen>>,
```

---

### ✅ Good Usage of Entity in Query:
```rust
// Need both Entity (to despawn) and component data
enemy_query: Query<(Entity, &Enemy)>,
// ...
for (entity, enemy) in enemy_query.iter() {
    if enemy.health <= 0 {
        commands.entity(entity).despawn(); // Need Entity for this!
    }
}
```

---

## Summary Table

| Query Type | What You Get | Use Case |
|------------|--------------|----------|
| `Query<&T>` | Component data | Just read/write data |
| `Query<Entity, With<T>>` | Entity IDs | Despawn, reference entities |
| `Query<(Entity, &T)>` | Both | Need data + despawn |
| `Query<(), With<T>>` | Existence check | Count entities |
| `Single<&T>` | Exactly one | Single entity (player) |

---

## When Do You Need Entity?

**You NEED `Entity` when:**
1. **Despawning** - `commands.entity(entity).despawn()`
2. **Passing to other systems** - Storing entity IDs in events or resources
3. **Looking up by ID** - Retrieving an entity by its stored ID

**You DON'T need `Entity` when:**
1. Just reading/modifying component data
2. Just counting entities
3. Just checking if entities exist

---

## Key Takeaway

- **Entity** = Unique ID (handle/reference)
- `Query<Entity, With<T>>` = "Give me IDs of all entities with component T"
- `Query<&T>` = "Give me the T component data from all entities"
- `Query<(Entity, &T)>` = "Give me both IDs and data"

**Rule of thumb:** If you don't need to despawn or reference the entity later, don't include it in your query!