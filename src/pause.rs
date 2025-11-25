use bevy::prelude::*;
use crate::resources::{GameState, ShouldResetOnStart};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_input.run_if(in_state(GameState::Running)))
           .add_systems(Update, resume_input.run_if(in_state(GameState::Paused)))
           .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
           .add_systems(OnExit(GameState::Paused), cleanup_pause_menu);
    }
}

#[derive(Component)]
struct PauseMenuUi;

fn pause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn resume_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut should_reset: Option<ResMut<ShouldResetOnStart>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Running);
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        // Mark that the next time we start the game from the menu it should be a fresh init
        if let Some(mut flag) = should_reset {
            flag.0 = true;
        }
        next_state.set(GameState::Menu);
    }
}

fn setup_pause_menu(mut commands: Commands) {
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
        PauseMenuUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("PAUSED"),
            TextFont {
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 0.8, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
        ));
        
        parent.spawn((
            Text::new("Press ESC to Resume"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ));
        
        parent.spawn((
            Text::new("Press Q to Quit to Menu"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ));
    });
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
