# Hotline Miami Clone in Bevy 0.17.3

A minimalistic Hotline Miami clone built with Rust and Bevy 0.17.3, serving as a learning project for practicing Rust basics through game development.

## Table of Contents
- [Project Overview](#project-overview)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-architecture)
- [ECS Architecture](#ecs-architecture)
  - [Components](#components)
  - [Systems](#systems)
  - [Resources](#resources)
- [Assets](#assets)
- [Game Mechanics](#game-mechanics)
- [Controls](#controls)
- [Development Rules](#development-rules)
- [Building and Running](#building-and-running)

---

## Project Overview
This is a top-down shooter game inspired by Hotline Miami, featuring:
- Player movement and combat
- Enemy AI with chase behavior
- AABB collision detection and resolution
- Level loading via LDtk editor
- Audio feedback for combat actions
- Victory/game over states
- Game restart functionality

---

## Technology Stack

| Dependency | Version | Purpose |
|------------|---------|---------|
| **Bevy** | 0.17.3 | Game engine and ECS framework |
| **bevy_ecs_ldtk** | 0.13 | LDtk level format integration |

**Critical**: This project strictly uses Bevy 0.17.3. All APIs and patterns are compatible with this version only.

---

## Project Architecture

```
src/
├── main.rs          ← App initialization, system registration, plugin setup
├── components.rs    ← All ECS component definitions
├── constants.rs     ← Game balance values and configuration
├── resources.rs     ← Resource definitions (GameState)
├── player.rs        ← Player-specific systems (movement, attack)
├── enemy.rs         ← Enemy-specific systems (AI, damage)
├── collision.rs     ← Collision detection and resolution systems
├── systems.rs       ← Utility systems (cleanup, restart, cooldowns)
├── ui.rs            ← UI and game state display systems
├── setup.rs         ├── World/level initialization
├── utils.rs         └── Helper functions (restart_game)
└── bin/
    └── ldtk_viewer.rs ← Standalone LDtk level viewer tool
```

---

## ECS Architecture

### Components

#### `GameEntity`
**Type**: Marker Component
**Purpose**: Identifies entities that belong to the game world (used for cleanup on restart)

```rust
#[derive(Component)]
pub struct GameEntity;
```

---

#### `Player`
**Type**: Data Component
**Purpose**: Marks the player entity and tracks health

```rust
#[derive(Component)]
pub struct Player {
    pub health: i32,
}
```

**Default Health**: 3

---

#### `Enemy`
**Type**: Data Component
**Purpose**: Marks enemy entities and tracks health

```rust
#[derive(Component)]
pub struct Enemy {
    pub health: i32,
}
```

**Default Health**: 1

---

#### `AttackCooldown`
**Type**: Data Component
**Purpose**: Stores cooldown timer for attack actions

```rust
#[derive(Component)]
pub struct AttackCooldown(pub Timer);
```

**Default Duration**: 0.5 seconds (defined in `ATTACK_COOLDOWN` constant)

---

#### `TextScreen`
**Type**: Marker Component
**Purpose**: Identifies UI text overlay entities (victory/game over screens)

```rust
#[derive(Component)]
pub struct TextScreen;
```

---

#### `Collider`
**Type**: Data Component
**Purpose**: Defines AABB collision shape using half-extents

```rust
#[derive(Component)]
pub struct Collider {
    pub half_size: Vec2,
}
```

**Used By**: Player, enemies, and walls

---

#### `Wall`
**Type**: Marker Component
**Purpose**: Identifies static collision geometry

```rust
#[derive(Component)]
pub struct Wall;
```

**Note**: Used in query filters to separate static from dynamic entities

---

### Systems

#### Player Systems (`src/player.rs`)

##### `player_movement`
**Query**: `Single<&mut Transform, With<Player>>`
**Resources**: `ButtonInput<KeyCode>`, `Time`
**Purpose**: Handles WASD/Arrow key movement with normalized direction vector

**Input**:
- W/Up Arrow: Move up
- S/Down Arrow: Move down
- A/Left Arrow: Move left
- D/Right Arrow: Move right

**Speed**: 200 pixels/second (`PLAYER_SPEED` constant)

**Dependencies**: Must run before `player_wall_collision`

---

##### `player_attack`
**Queries**:
- `Single<(Entity, &Transform, &mut AttackCooldown), With<Player>>`
- `Query<(Entity, &Transform, &mut Enemy)>`

**Resources**: `ButtonInput<MouseButton>`, `Time`, `GameState`, `AssetServer`
**Purpose**: Handles melee attack on left mouse click

**Mechanics**:
- Checks attack cooldown
- Calculates attack direction from player to mouse cursor
- Damages all enemies within `ATTACK_RANGE` (50px) in attack direction
- Plays attack sound effect
- Uses dot product to verify enemy is in attack cone (forward only)

---

#### Enemy Systems (`src/enemy.rs`)

##### `enemy_ai`
**Queries**:
- `Single<&Transform, With<Player>>`
- `Query<(&mut Transform, &Enemy), Without<Player>>`

**Resources**: `Time`, `GameState`
**Purpose**: Simple chase AI that moves enemies toward player

**Behavior**:
- Only moves when enemy.health > 0
- Calculates direction vector to player
- Normalizes direction and applies movement
- Stops if game over

**Speed**: 100 pixels/second (`ENEMY_SPEED` constant)

**Dependencies**: Must run before `enemy_wall_collision`

---

##### `enemy_damage`
**Queries**:
- `Single<(Entity, &Transform, &mut Player)>`
- `Query<(&Transform, &Enemy), Without<Player>>`
- `Query<Entity, With<Enemy>>`
- `Query<Entity, With<GameEntity>>`
- `Query<&TextScreen>`

**Resources**: `Time`, `GameState`, `AssetServer`
**Purpose**: Handles enemy-to-player damage

**Mechanics**:
- Checks distance from each alive enemy to player
- If within `ENEMY_DAMAGE_RANGE` (40px), applies damage every 1 second
- Plays hit sound effect on damage
- Triggers game restart when player dies

**Damage Rate**: 1 damage per second per nearby enemy

---

#### Collision Systems (`src/collision.rs`)

##### `push_out_of_wall` (private helper)
**Purpose**: Core collision resolution using SAT (Separating Axis Theorem)

**Algorithm**:
1. Create AABB from mover position and half-size
2. Create AABB from wall position and half-size
3. Check intersection using `IntersectsVolume`
4. If intersecting:
   - Calculate overlap on X and Y axes
   - Push out on axis with smallest overlap
   - Direction determined by relative position

---

##### `player_wall_collision`
**Queries**:
- `Query<(&mut Transform, &Collider), With<Player>>`
- `Query<(&Transform, &Collider), (With<Wall>, Without<Player>)>`

**Purpose**: Resolves player collision with all walls

**Dependencies**: Runs `.after(player_movement)`

---

##### `enemy_wall_collision`
**Queries**:
- `Query<(&mut Transform, &Collider), (With<Enemy>, Without<Wall>)>`
- `Query<(&Transform, &Collider), (With<Wall>, Without<Enemy>)>`

**Purpose**: Resolves each enemy's collision with all walls

**Dependencies**: Runs `.after(enemy_ai)`

**Note**: Uses `Without<>` filters to prevent query overlap conflicts

---

#### Utility Systems (`src/systems.rs`)

##### `update_attack_cooldowns`
**Query**: `Query<&mut AttackCooldown>`
**Resources**: `Time`
**Purpose**: Ticks all attack cooldown timers each frame

---

##### `cleanup_dead_entities`
**Queries**:
- `Query<(Entity, &Enemy), With<Enemy>>`

**Resources**: `GameState`
**Purpose**: Despawns dead enemies and decrements enemy counter

**Condition**: Enemies with `health <= 0` are removed

---

##### `check_restart_button`
**Queries**:
- `Query<Entity, With<GameEntity>>`
- `Query<Entity, With<Enemy>>`

**Resources**: `ButtonInput<KeyCode>`, `GameState`, `AssetServer`
**Purpose**: Triggers full game restart on R key press

**Behavior**: Despawns all game entities and respawns level

---

#### UI Systems (`src/ui.rs`)

##### `check_game_state`
**Queries**:
- `Query<&TextScreen>`

**Resources**: `GameState`
**Purpose**: Displays victory screen when all enemies defeated

**Trigger Condition**:
- `enemies_remaining <= 0`
- `game_over == false`
- `victory == false`
- No text screen currently exists

**Display**: Centered "YOU WIN!" text

---

### Resources

#### `GameState`
**Type**: Resource
**Purpose**: Tracks global game state across systems

```rust
#[derive(Resource)]
pub struct GameState {
    pub game_over: bool,           // True when player has died
    pub victory: bool,             // True when all enemies defeated
    pub damage_timer: f32,         // Accumulates time for damage cooldown
    pub enemies_remaining: usize,  // Count of alive enemies
}
```

**Modified By**:
- `enemy_damage` - updates damage_timer, triggers game_over
- `cleanup_dead_entities` - decrements enemies_remaining
- `check_game_state` - sets victory
- `setup_initial_state` (helper) - resets all values

---

## Assets

### Level Data
| File | Size | Format | Purpose |
|------|------|--------|---------|
| `levels/HotlineMiamiLikeWorld.ldtk` | 531KB | LDtk project | Main level definitions |

**LDtk Integration**: Levels loaded via `bevy_ecs_ldtk` plugin using `LdtkWorldBundle`

---

### Audio Assets
| File | Size | Format | Trigger |
|------|------|--------|---------|
| `player_attack.ogg` | 4.0KB | OGG Vorbis | Player left-click attack |
| `enemy_hit.ogg` | 3.9KB | OGG Vorbis | Enemy damages player (every 1s) |

**Playback**: One-shot `AudioPlayer` spawning on sound events

---

### Sprite Assets (Active)
| File | Size | Used By | Dimensions |
|------|------|---------|------------|
| `player.png` | 291B | Player entity | ~32x32 (scaled by `PLAYER_SIZE`) |
| `enemy.png` | 300B | Enemy entities | ~32x32 (scaled by `ENEMY_SIZE`) |

**Note**: Actual sprite sizes may vary; colliders use `PLAYER_SIZE`/`ENEMY_SIZE` constants (32.0)

---

### Sprite Assets (Available - Not Integrated)
**Location**: `assets/Characters_free/`

Free character sprite pack with multiple characters and animations:

| Character | Files Available | Animations |
|-----------|-----------------|------------|
| **Adam** | Adam_16x16.png, Adam_idle_16x16.png, Adam_idle_anim_16x16.png, Adam_phone_16x16.png, Adam_run_16x16.png, Adam_sit_16x16.png, Adam_sit2_16x16.png, Adam_sit3_16x16.png | Idle, run, sit variations, phone |
| **Alex** | Alex_16x16.png, Alex_idle_16x16.png, Alex_idle_anim_16x16.png, Alex_phone_16x16.png, Alex_run_16x16.png, Alex_sit_16x16.png, Alex_sit2_16x16.png, Alex_sit3_16x16.png | Idle, run, sit variations, phone |
| **Amelia** | Amelia_16x16.png, Amelia_idle_16x16.png, Amelia_idle_anim_16x16.png, Amelia_phone_16x16.png, Amelia_run_16x16.png, Amelia_sit_16x16.png, Amelia_sit2_16x16.png, Amelia_sit3_16x16.png | Idle, run, sit variations, phone |
| **Bob** | Bob_16x16.png, Bob_idle_16x16.png, Bob_idle_anim_16x16.png, Bob_phone_16x16.png, Bob_run_16x16.png, Bob_sit_16x16.png, Bob_sit2_16x16.png, Bob_sit3_16x16.png | Idle, run, sit variations, phone |

**Source**: [Itch.io "Characters_free" pack](https://ansimuz.itch.io/free-game-assets)

---

### Tile/Interior Assets (Available - Not Integrated)
**Location**: `assets/Interiors_free/`

Free interior tileset pack for building room environments:

| Resolution | Files | Purpose |
|------------|-------|---------|
| **16x16** | Interiors_free_16x16.png (134KB), Room_Builder_free_16x16.png (11KB) | Small tile interiors |
| **32x32** | Interiors_free_32x32.png (179KB), Room_Builder_free_32x32.png (17KB) | Medium tile interiors |
| **48x48** | (various files) | Large tile interiors |

**Source**: [Itch.io "Interiors_free" pack](https://ansimuz.itch.io/interiors-free)

**Potential Use**: Replace procedural wall generation with tile-based level design

---

### Legacy Assets
**Location**: `assets/Old/`

Older tilesets in multiple resolutions (16x16, 32x32, 48x48):
- Idle animations
- Run animations (horizontal)
- Tileset variants

**Status**: Superseded by newer asset packs; kept for reference

---

## Game Mechanics

### Combat System

#### Player Attack
1. Player clicks left mouse button
2. System checks `AttackCooldown` (0.5s default)
3. If ready:
   - Reset cooldown
   - Calculate direction: `(mouse_position - player_position).normalize()`
   - Find all enemies within `ATTACK_RANGE` (50px)
   - Check if enemy is in front: `dot(enemy_direction, attack_direction) > 0`
   - Apply 1 damage to valid targets
   - Play `player_attack.ogg` sound

#### Enemy Damage
1. Each frame, check distance from each alive enemy to player
2. If distance ≤ `ENEMY_DAMAGE_RANGE` (40px):
   - Accumulate time in `GameState.damage_timer`
   - Every 1.0 second: apply 1 damage to player
   - Play `enemy_hit.ogg` sound
   - If player.health ≤ 0: trigger game restart

---

### Collision System

**Type**: Custom AABB (Axis-Aligned Bounding Box)
**Library**: `bevy::math::bounding` (built-in to Bevy 0.17.3)

**Workflow**:
1. Movement system updates entity position
2. Collision system runs (ordered `.after()` movement)
3. For each dynamic entity (player/enemies):
   - Create AABB from position and `Collider.half_size`
   - Check intersection with each wall AABB
   - If intersecting: push out on axis of least overlap
4. Entity ends frame at valid position (no wall overlap)

**No Physics Engine**: Chose manual implementation over external crates (avian2d, bevy_rapier2d) for learning purposes and simplicity

---

### Game Flow

#### Normal Play
1. Level loads via LDtk
2. Player and enemies spawn at defined positions
3. Player moves and attacks to defeat enemies
4. Enemies chase and damage player on contact
5. When all enemies die → "YOU WIN!" screen

#### Game Over
1. Player health reaches 0
2. Game immediately restarts (respawns all entities)
3. No separate game over screen currently

#### Manual Restart
- Press **R** at any time to restart

---

## Controls

| Action | Input |
|--------|-------|
| Move Up | W or ↑ |
| Move Down | S or ↓ |
| Move Left | A or ← |
| Move Right | D or → |
| Attack | Left Mouse Button |
| Restart Game | R |

---

## Development Rules

### Bevy Version Compliance
- **STRICTLY USE BEVY 0.17.3 ONLY**
- All code must be compatible with Bevy 0.17.3 APIs
- Do not use features from later versions
- Always verify API existence before suggesting changes

### Code Guidelines
- Keep code simple and educational
- Use clear, descriptive names
- Add comments explaining Rust concepts
- Demonstrate proper ownership, borrowing, and lifetimes
- Follow ECS patterns properly (Entities, Components, Systems)

### Query Patterns (from `bevy_cheatbook.md`)
- Use `Single<T>` for queries expected to return exactly one result (e.g., player)
- Avoid fetching Entity IDs in queries when not needed
- Use `With<>` and `Without<>` filters to prevent query conflicts
- Order systems explicitly with `.after()` when dependencies exist

---

## Building and Running

### Prerequisites
- Rust 2024 edition
- Cargo package manager

### Development Build
```bash
cargo run
```

### Release Build
```bash
cargo run --release
```

### LDtk Viewer (Standalone Tool)
```bash
cargo run --bin ldtk_viewer
```

**Note**: Build optimizations configured in `Cargo.toml`:
```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

---

## Known Issues / Future Improvements

### Potential Enhancements
- [ ] Add visual attack effects (swing animation, hit feedback)
- [ ] Add score system
- [ ] Add different enemy types with varied behaviors
- [ ] Add more complex level geometry
- [ ] Implement level progression system
- [ ] Add proper game over screen
- [ ] Display on-screen restart instructions
- [ ] Integrate animation from character sprite packs
- [ ] Replace procedural walls with tile-based levels
- [ ] Add weapon pickups and variety
- [ ] Implement enemy patrol/wander AI
- [ ] Add particle effects for combat feedback

---

## Credits

**Game Engine**: [Bevy Engine](https://bevyengine.org/) v0.17.3

**Level Tool**: [LDtk](https://ldtk.io/) with [bevy_ecs_ldtk](https://github.com/Trouv/bevy_ecs_ldtk) v0.13

**Assets**:
- Characters: [Free Game Assets by Ansimuz](https://ansimuz.itch.io/free-game-assets)
- Interiors: [Interiors Free by Ansimuz](https://ansimuz.itch.io/interiors-free)

---

## License

This is a learning project. Assets are used under their respective free licenses from Itch.io.
