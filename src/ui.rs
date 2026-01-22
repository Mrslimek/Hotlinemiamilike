use bevy::prelude::*;

use crate::components::{GameEntity, TextScreen};
use crate::resources::GameState;

pub fn check_game_state(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    text_screen_query: Query<&TextScreen>,
) {
    if game_state.enemies_remaining <= 0
        && !game_state.game_over
        && !game_state.victory
        && text_screen_query.count() == 0
    {
        let text_style = TextFont {
            font_size: 42.0,
            ..default()
        };

        commands
            .spawn((
                Node {
                    height: percent(100),
                    width: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                TextScreen,
                GameEntity,
            ))
            .with_child((Text::new("YOU WIN!"), text_style.clone()));
        game_state.victory = true;
    }
}
