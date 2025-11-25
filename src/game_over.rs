use bevy::prelude::*;
use crate::resources::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over)
           .add_systems(Update, game_over_input.run_if(in_state(GameState::GameOver)))
           .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
           // Also reset game state when entering Running from Menu/Difficulty selection
           .add_systems(OnEnter(GameState::Running), reset_game_on_start);
    }
}

#[derive(Component)]
struct GameOverUi;

fn setup_game_over(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        GameOverUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
        ));
        
        parent.spawn((
            Text::new("Press SPACE to Restart"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
        ));
    });
}

fn game_over_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    enemy_query: Query<Entity, With<crate::enemy::Enemy>>,
    mut player_query: Query<&mut crate::player::Ship, With<crate::player::Player>>,
    mut wave: ResMut<crate::resources::Wave>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        if let Ok(mut ship) = player_query.get_single_mut() {
            *ship = crate::player::Ship::default();
        }
        
        *wave = crate::resources::Wave::default();
        
        next_state.set(GameState::Running);
    }
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_game_on_start(
    mut commands: Commands,
    enemy_query: Query<Entity, With<crate::enemy::Enemy>>,
    mut player_query: Query<(&mut crate::player::Ship, &mut Transform), With<crate::player::Player>>,
    mut wave: ResMut<crate::resources::Wave>,
    mut typing_buffer: ResMut<crate::ui::TypingBuffer>,
) {
    // Despawn any existing enemies
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Reset player ship stats and transform
    if let Ok((mut ship, mut transform)) = player_query.get_single_mut() {
        *ship = crate::player::Ship::default();
        *transform = Transform::default();
    }

    // Reset wave/resource state
    *wave = crate::resources::Wave::default();

    // Clear typing buffer
    typing_buffer.text.clear();
}
