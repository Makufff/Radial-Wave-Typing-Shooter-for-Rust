use bevy::prelude::*;
use crate::resources::{GameState, Difficulty};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Difficulty>()
           .add_systems(OnEnter(GameState::Menu), setup_main_menu)
           .add_systems(Update, main_menu_input.run_if(in_state(GameState::Menu)))
           .add_systems(OnExit(GameState::Menu), cleanup_menu)
           .add_systems(OnEnter(GameState::DifficultySelect), setup_difficulty_menu)
           .add_systems(Update, difficulty_menu_input.run_if(in_state(GameState::DifficultySelect)))
           .add_systems(OnExit(GameState::DifficultySelect), cleanup_difficulty_menu);
    }
}

#[derive(Component)]
struct MainMenuUi;

#[derive(Component)]
struct DifficultyMenuUi;

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        MainMenuUi,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("RADIAL WAVE TYPING SHOOTER"),
            TextFont {
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 0.8, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
        ));
        
        // Play Button
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.6, 0.8)),
        )).with_children(|button| {
            button.spawn((
                Text::new("PLAY"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
        
        // Instructions
        parent.spawn((
            Text::new("Press SPACE or ENTER to Play"),
            TextFont {
                font_size: 25.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            },
        ));
    });
}

fn main_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::DifficultySelect);
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_difficulty_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        DifficultyMenuUi,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("SELECT DIFFICULTY"),
            TextFont {
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 0.8, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
        ));
        
        // Easy Button
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.7, 0.0)),
        )).with_children(|button| {
            button.spawn((
                Text::new("EASY MODE"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));
            button.spawn((
                Text::new("(lowercase only)"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
        
        // Hard Button
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.0, 0.0)),
        )).with_children(|button| {
            button.spawn((
                Text::new("HARD MODE"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));
            button.spawn((
                Text::new("(Mixed Case)"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
        
        // Instructions
        parent.spawn((
            Text::new("Press 1 or E for Easy  |  Press 2 or H for Hard"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            },
        ));
    });
}

fn difficulty_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut difficulty: ResMut<Difficulty>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) || keyboard_input.just_pressed(KeyCode::KeyE) {
        *difficulty = Difficulty::Easy;
        next_state.set(GameState::Running);
    } else if keyboard_input.just_pressed(KeyCode::Digit2) || keyboard_input.just_pressed(KeyCode::KeyH) {
        *difficulty = Difficulty::Hard;
        next_state.set(GameState::Running);
    }
}

fn cleanup_difficulty_menu(mut commands: Commands, query: Query<Entity, With<DifficultyMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

