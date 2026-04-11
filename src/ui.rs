use bevy::prelude::*;

// use crate::components::{GameEntity, TextScreen};
// use crate::resources::GameState;
use crate::score::ScoreState;
use crate::settings::GameSettings;

/// Маркирующий компонент для UI entities
#[derive(Component)]
pub struct GameUi;

// pub fn check_game_state(
//     mut commands: Commands,
//     mut game_state: ResMut<GameState>,
//     text_screen_query: Query<&TextScreen>,
// ) {
//     if game_state.enemies_remaining <= 0
//         && !game_state.game_over
//         && !game_state.victory
//         && text_screen_query.count() == 0
//     {
//         let text_style = TextFont {
//             font_size: 42.0,
//             ..default()
//         };

//         commands
//             .spawn((
//                 Node {
//                     height: percent(100),
//                     width: percent(100),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 BackgroundColor(Color::BLACK),
//                 TextScreen,
//                 GameEntity,
//             ))
//             .with_child((Text::new("YOU WIN!"), text_style.clone()));
//         game_state.victory = true;
//     }
// }

pub fn setup_ui(mut commands: Commands) {
    // Создать отдельную камеру для UI (поверх игровой)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,  // Поверх игровой камеры (order: 0)
            ..default()
        },
    ));

    // Создать контейнер для UI
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            GameUi,
        ))
        .with_children(|parent| {
            // SCORE (верхний левый угол)
            parent.spawn((
                Text::new("SCORE: 0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                },
            ));

            // COMBO (ниже score)
            parent.spawn((
                Text::new("COMBO: ×1.0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.0)), // Золотой цвет
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(50.0),
                    left: Val::Px(20.0),
                    ..default()
                },
            ));

            // TIME (верхний правый угол)
            parent.spawn((
                Text::new("TIME: 0.0s"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..default()
                },
            ));
        });
}

pub fn process_ui_updates(
    score: Res<ScoreState>,
    settings: Res<GameSettings>,
    mut query: Query<&mut Text>,
) {
    if !settings.ui.show_ui {
        return;  // Не обновлять если UI выключен
    }

    let mut texts = query.iter_mut();

    // Обновить SCORE (первый текст)
    if let Some(mut text) = texts.next() {
        text.0 = format!("SCORE: {}", score.current_score);
    }

    // Обновить COMBO (второй текст)
    if let Some(mut text) = texts.next() {
        text.0 = format!("COMBO: ×{:.1}", score.combo_multiplier);
    }

    // Обновить TIME (третий текст)
    if let Some(mut text) = texts.next() {
        text.0 = format!("TIME: {:.1}s", score.room_time);
    }
}
