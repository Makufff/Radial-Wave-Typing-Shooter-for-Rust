use bevy::prelude::*;
use crate::resources::{GameState, Difficulty, GameSettings, MenuSelection};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Difficulty>()
           .init_resource::<GameSettings>()
           .insert_resource(MenuSelection::new(2))
           .add_systems(OnEnter(GameState::Menu), (reset_main_menu_selection, setup_main_menu).chain())
           .add_systems(Update, main_menu_input.run_if(in_state(GameState::Menu)))
           .add_systems(OnExit(GameState::Menu), cleanup_menu)
           .add_systems(OnEnter(GameState::Settings), setup_settings_menu)
           .add_systems(Update, settings_menu_input.run_if(in_state(GameState::Settings)))
           .add_systems(OnExit(GameState::Settings), cleanup_settings_menu)
           .add_systems(OnEnter(GameState::DifficultySelect), (reset_difficulty_menu_selection, setup_difficulty_menu).chain())
           .add_systems(Update, difficulty_menu_input.run_if(in_state(GameState::DifficultySelect)))
           .add_systems(OnExit(GameState::DifficultySelect), cleanup_difficulty_menu);
    }
}

#[derive(Component)]
struct MainMenuUi;

#[derive(Component)]
struct SettingsMenuUi;

#[derive(Component)]
struct DifficultyMenuUi;

#[derive(Component)]
struct MenuItem {
    index: usize,
}

fn reset_main_menu_selection(mut menu_selection: ResMut<MenuSelection>) {
    *menu_selection = MenuSelection::new(2);
}

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
            MenuItem { index: 0 },
            Node {
                width: Val::Px(300.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.4, 0.5)),
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
        
        // Settings Button
        parent.spawn((
            MenuItem { index: 1 },
            Node {
                width: Val::Px(300.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.4, 0.5)),
        )).with_children(|button| {
            button.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
        
        // Instructions
        parent.spawn((
            Text::new("Use Arrow Keys to Select\nPress SPACE or ENTER to Confirm"),
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

fn main_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut menu_selection: ResMut<MenuSelection>,
    mut menu_items: Query<(&MenuItem, &mut BackgroundColor)>,
) {
    let mut selection_changed = false;
    
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        menu_selection.move_up();
        selection_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        menu_selection.move_down();
        selection_changed = true;
    }
    
    if selection_changed {
        // Update visual highlighting
        for (item, mut bg_color) in menu_items.iter_mut() {
            if item.index == menu_selection.selected_index {
                *bg_color = BackgroundColor(Color::srgb(0.0, 0.8, 1.0)); // Highlighted
            } else {
                if item.index == 0 {
                    *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.5)); // Play button normal
                } else {
                    *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.5)); // Settings button normal
                }
            }
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter) {
        match menu_selection.selected_index {
            0 => next_state.set(GameState::DifficultySelect),
            1 => next_state.set(GameState::Settings),
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_settings_menu(mut commands: Commands, settings: Res<GameSettings>) {
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
        SettingsMenuUi,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("SETTINGS"),
            TextFont {
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 0.8, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ));
        
        // Controls Section
        parent.spawn((
            Node {
                width: Val::Px(600.0),
                padding: UiRect::all(Val::Px(20.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.7)),
        )).with_children(|section| {
            section.spawn((
                Text::new("CONTROLS"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.8, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));
            
            let controls = [
                "Type words to target enemies",
                "SPACE/ENTER - Submit word",
                "ESC - Pause game",
                "Movement - Arrow keys",
            ];
            
            for control in controls {
                section.spawn((
                    Text::new(control),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    Node {
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                ));
            }
        });
        
        // Volume Section
        parent.spawn((
            Node {
                width: Val::Px(600.0),
                padding: UiRect::all(Val::Px(20.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.7)),
        )).with_children(|section| {
            section.spawn((
                Text::new("AUDIO"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.8, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));
            
            section.spawn((
                Text::new(format!("Master Volume: {:.0}%", settings.master_volume * 100.0)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
            ));
            
            section.spawn((
                Text::new(format!("SFX Volume: {:.0}%", settings.sfx_volume * 100.0)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
            ));
            
            section.spawn((
                Text::new(format!("Music Volume: {:.0}%", settings.music_volume * 100.0)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
            ));
        });
        
        // Back Button
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        )).with_children(|button| {
            button.spawn((
                Text::new("BACK"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
        
        // Instructions
        parent.spawn((
            Text::new("Press ESC or B to go back"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
        ));
    });
}

fn settings_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::KeyB) {
        next_state.set(GameState::Menu);
    }
}

fn cleanup_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_difficulty_menu_selection(mut menu_selection: ResMut<MenuSelection>) {
    *menu_selection = MenuSelection::new(2);
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
            MenuItem { index: 0 },
            Node {
                width: Val::Px(400.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.9, 0.0)), // Highlighted by default
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
            MenuItem { index: 1 },
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
            Text::new("Use ↑↓ Arrow Keys to Select  |  Press SPACE or ENTER to Confirm"),
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
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut difficulty: ResMut<Difficulty>,
    mut menu_selection: ResMut<MenuSelection>,
    mut menu_items: Query<(&MenuItem, &mut BackgroundColor), With<MenuItem>>,
    enemy_query: Query<Entity, With<crate::enemy::Enemy>>,
    mut player_query: Query<&mut crate::player::Ship, With<crate::player::Player>>,
    mut wave: ResMut<crate::resources::Wave>,
) {
    let mut selection_changed = false;

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        menu_selection.move_up();
        selection_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        menu_selection.move_down();
        selection_changed = true;
    }

    if selection_changed {
        // Update visual highlighting
        for (item, mut bg_color) in menu_items.iter_mut() {
            if item.index == menu_selection.selected_index {
                if item.index == 0 {
                    *bg_color = BackgroundColor(Color::srgb(0.0, 0.9, 0.0)); // Easy highlighted
                } else {
                    *bg_color = BackgroundColor(Color::srgb(1.0, 0.0, 0.0)); // Hard highlighted
                }
            } else {
                if item.index == 0 {
                    *bg_color = BackgroundColor(Color::srgb(0.0, 0.7, 0.0)); // Easy normal
                } else {
                    *bg_color = BackgroundColor(Color::srgb(0.8, 0.0, 0.0)); // Hard normal
                }
            }
        }
    }

    // Keep keyboard shortcuts for backward compatibility
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        *difficulty = Difficulty::Easy;
        // Reset game state (despawn enemies, reset player and wave) before starting
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        if let Ok(mut ship) = player_query.get_single_mut() {
            *ship = crate::player::Ship::default();
        }
        *wave = crate::resources::Wave::default();
        next_state.set(GameState::Running);
    } else if keyboard_input.just_pressed(KeyCode::KeyH) {
        *difficulty = Difficulty::Hard;
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        if let Ok(mut ship) = player_query.get_single_mut() {
            *ship = crate::player::Ship::default();
        }
        *wave = crate::resources::Wave::default();
        next_state.set(GameState::Running);
    } else if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter) {
        match menu_selection.selected_index {
            0 => *difficulty = Difficulty::Easy,
            1 => *difficulty = Difficulty::Hard,
            _ => {}
        }
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

fn cleanup_difficulty_menu(mut commands: Commands, query: Query<Entity, With<DifficultyMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

