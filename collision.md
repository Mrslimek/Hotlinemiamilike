# Коллизия в Bevy 0.17.3

## Важное: Bevy не имеет встроенного физического движка

В отличие от Unity или Godot, Bevy **не поставляется** со встроенной системой физики или коллизий.
Для добавления коллизий есть два пути:

1. **Ручная реализация AABB** — используя встроенный модуль `bevy::math::bounding` (наш выбор)
2. **Внешние плагины** — например `avian2d` или `bevy_rapier2d`

Мы выбрали ручную реализацию, потому что:
- Не требует дополнительных зависимостей
- Полностью контролируема
- Достаточна для простого top-down шутера
- Официально используется в примерах самого Bevy (см. `examples/games/breakout.rs`)

---

## Встроенные инструменты: `bevy::math::bounding`

Bevy 0.17.3 предоставляет модуль `bevy::math::bounding` с готовыми примитивами для обнаружения пересечений:

| Тип | Назначение |
|-----|-----------|
| `Aabb2d` | Прямоугольная ограничивающая рамка (Axis-Aligned Bounding Box) |
| `BoundingCircle` | Круглая ограничивающая область |
| `IntersectsVolume` | Трейт для проверки пересечения двух форм |

### Как создать `Aabb2d`:

```rust
use bevy::math::bounding::Aabb2d;

// Первый аргумент — центр, второй — half-size (полуразмер)
let aabb = Aabb2d::new(transform.translation.truncate(), half_size);
```

### Как проверить пересечение:

```rust
use bevy::math::bounding::IntersectsVolume;

if aabb_a.intersects(&aabb_b) {
    // Пересечение есть — нужно разрешить коллизию
}
```

Источник: [официальный пример breakout.rs, Bevy v0.17.3](https://github.com/bevyengine/bevy/blob/v0.17.3/examples/games/breakout.rs)

---

## Компоненты, которые нужно добавить

В Bevy коллизия строится на компонентах ECS. Нам понадобятся два новых компонента в `src/components.rs`.

### `Collider` — хранит размер хитбокса

```rust
#[derive(Component)]
pub struct Collider {
    pub half_size: Vec2,
}
```

`half_size` — это **половина** ширины и высоты объекта. Например, для спрайта 32×32:
```rust
Collider { half_size: Vec2::splat(16.0) }
```

### `Wall` — маркер статических стен

```rust
#[derive(Component)]
pub struct Wall;
```

Это пустой компонент-маркер. Он нужен для того, чтобы в запросах (Query) можно было отдельно выбирать стены от динамических объектов.

---

## Алгоритм разрешения коллизии (Push-Out)

Когда два AABB пересекаются, нужно "вытолкнуть" движущийся объект из статического.
Алгоритм:

1. Вычислить вектор от центра стены до центра объекта (`diff`)
2. Вычислить перекрытие по каждой оси (`overlap_x`, `overlap_y`)
3. Вытолкнуть по оси с **меньшим** перекрытием (принцип SAT — Separating Axis Theorem)

```rust
fn push_out_of_wall(mover_pos: &mut Vec3, mover_half: Vec2, wall_pos: Vec3, wall_half: Vec2) {
    let mover_aabb = Aabb2d::new(mover_pos.truncate(), mover_half);
    let wall_aabb  = Aabb2d::new(wall_pos.truncate(),  wall_half);

    if !mover_aabb.intersects(&wall_aabb) {
        return;
    }

    let diff      = mover_pos.truncate() - wall_pos.truncate();
    let overlap_x = (mover_half.x + wall_half.x) - diff.x.abs();
    let overlap_y = (mover_half.y + wall_half.y) - diff.y.abs();

    if overlap_x < overlap_y {
        mover_pos.x += overlap_x * diff.x.signum();
    } else {
        mover_pos.y += overlap_y * diff.y.signum();
    }
}
```

---

## Системы коллизии (`src/collision.rs`)

Для каждого типа динамического объекта создаётся своя система:

### Игрок vs. стены

```rust
pub fn player_wall_collision(
    mut player_query: Query<(&mut Transform, &Collider), With<Player>>,
    wall_query: Query<(&Transform, &Collider), (With<Wall>, Without<Player>)>,
) {
    let Ok((mut player_transform, player_collider)) = player_query.get_single_mut() else {
        return;
    };

    for (wall_transform, wall_collider) in wall_query.iter() {
        push_out_of_wall(
            &mut player_transform.translation,
            player_collider.half_size,
            wall_transform.translation,
            wall_collider.half_size,
        );
    }
}
```

### Враги vs. стены

```rust
pub fn enemy_wall_collision(
    mut enemy_query: Query<(&mut Transform, &Collider), (With<Enemy>, Without<Wall>)>,
    wall_query: Query<(&Transform, &Collider), (With<Wall>, Without<Enemy>)>,
) {
    for (mut enemy_transform, enemy_collider) in enemy_query.iter_mut() {
        for (wall_transform, wall_collider) in wall_query.iter() {
            push_out_of_wall(
                &mut enemy_transform.translation,
                enemy_collider.half_size,
                wall_transform.translation,
                wall_collider.half_size,
            );
        }
    }
}
```

**Важно про фильтры запросов**: Bevy требует, чтобы два запроса в одной системе не пересекались по мутабельным компонентам. Поэтому у стен добавлен фильтр `Without<Enemy>`, а у врагов — `Without<Wall>`.

---

## Регистрация систем в `main.rs`

Системы коллизии должны запускаться **после** систем движения, иначе за кадр объект окажется внутри стены до того, как его оттуда вытолкнут:

```rust
.add_systems(
    Update,
    (
        player_movement,
        enemy_ai,
        // Коллизии запускаются строго после движения
        player_wall_collision.after(player_movement),
        enemy_wall_collision.after(enemy_ai),
        // ... остальные системы
    ),
)
```

Метод `.after(system)` — это явное объявление порядка выполнения систем внутри одного расписания.

---

## Создание комнаты с 4 входами (`src/setup.rs`)

Комната состоит из **8 сегментов стен** — по 2 на каждую сторону с зазором (дверью) посередине.

### Математика разбивки стены

```
       [  left piece  ][  door  ][  right piece  ]
       |<-- piece_w -->|<-door_w->|<-- piece_w -->|
       |<------------- ROOM_HALF_SIZE * 2 -------->|
```

```rust
let piece_half = (ROOM_HALF_SIZE - DOOR_HALF_WIDTH) / 2.0;
let center_offset = DOOR_HALF_WIDTH + piece_half;
```

### Все 8 сегментов

```rust
let wall_segments: &[(Vec2, Vec2)] = &[
    // Верхняя стена
    (Vec2::new(-h_center_offset,  ROOM_HALF_SIZE), h_half_size),
    (Vec2::new( h_center_offset,  ROOM_HALF_SIZE), h_half_size),
    // Нижняя стена
    (Vec2::new(-h_center_offset, -ROOM_HALF_SIZE), h_half_size),
    (Vec2::new( h_center_offset, -ROOM_HALF_SIZE), h_half_size),
    // Левая стена
    (Vec2::new(-ROOM_HALF_SIZE,  v_center_offset), v_half_size),
    (Vec2::new(-ROOM_HALF_SIZE, -v_center_offset), v_half_size),
    // Правая стена
    (Vec2::new( ROOM_HALF_SIZE,  v_center_offset), v_half_size),
    (Vec2::new( ROOM_HALF_SIZE, -v_center_offset), v_half_size),
];
```

### Спавн каждого сегмента

```rust
commands.spawn((
    Sprite {
        color: WALL_COLOR,
        custom_size: Some(*half_size * 2.0), // custom_size принимает полный размер
        ..default()
    },
    Transform::from_xyz(position.x, position.y, 0.0),
    Wall,
    Collider { half_size: *half_size },
    GameEntity,
));
```

---

## Добавление `Collider` к динамическим объектам

Чтобы коллизия работала, `Collider` нужно добавить при спавне игрока и врагов:

```rust
// Игрок
commands.spawn((
    Sprite::from_image(asset_server.load("player.png")),
    Transform::from_xyz(0.0, 0.0, 0.0),
    Player { health: 3 },
    Collider { half_size: Vec2::splat(PLAYER_SIZE / 2.0) },
    // ...
));

// Враг
commands.spawn((
    Sprite::from_image(asset_server.load("enemy.png")),
    Transform::from_xyz(x, y, 0.0),
    Enemy { health: 1 },
    Collider { half_size: Vec2::splat(ENEMY_SIZE / 2.0) },
    // ...
));
```

---

## Почему `custom_size`, а не `Transform::scale` для стен?

В официальном примере Breakout стены используют `Transform::scale` для задания размера:
```rust
Transform {
    scale: wall_size.extend(1.0),
    ..default()
}
```

Мы используем `Sprite::custom_size`, потому что:
- `Transform::scale` масштабирует **всё дерево**, включая дочерние объекты
- `custom_size` задаёт размер только самого спрайта, не затрагивая трансформацию
- Размер в `Collider::half_size` проще синхронизировать с `custom_size`

---

## Итоговая структура файлов

```
src/
├── collision.rs      ← новый файл: системы обнаружения и разрешения коллизий
├── components.rs     ← добавлены: Collider, Wall
├── constants.rs      ← добавлены: WALL_COLOR, ROOM_HALF_SIZE, WALL_THICKNESS, DOOR_HALF_WIDTH
├── setup.rs          ← добавлены: Collider на игрока/врагов, spawn_room()
└── main.rs           ← добавлены: mod collision, регистрация систем с .after()
```
