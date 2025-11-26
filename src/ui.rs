use bevy::prelude::*;
use crate::player::{Player, Ship};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_background, setup_ui))
           .add_systems(Update, (animate_background, update_ui, update_typing_input).run_if(in_state(crate::resources::GameState::Running)))
           .init_resource::<TypingBuffer>();
    }
}

#[derive(Component)]
struct GridPoint {
    original_pos: Vec3,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HpText;

#[derive(Component)]
struct ComboText;

#[derive(Component)]
struct WaveText;

#[derive(Component)]
struct WeaponText;


#[derive(Component)]
struct TypingInputBox;

#[derive(Resource, Default)]
pub struct TypingBuffer {
    pub text: String,
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text::new("HP: 3"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        HpText,
    ));

    commands.spawn((
        Text::new("Combo: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(10.0),
            ..default()
        },
        ComboText,
    ));

    commands.spawn((
        Text::new("Wave: 1"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            right: Val::Px(10.0),
            ..default()
        },
        WaveText,
    ));

    commands.spawn((
        Text::new("Weapon: Blade"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        WeaponText,
    ));

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new(JustifyText::Center, LineBreak::AnyCharacter),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Percent(10.0),
            width: Val::Percent(80.0),
            max_width: Val::Percent(80.0),
            height: Val::Auto,
            min_height: Val::Px(60.0),
            padding: UiRect::all(Val::Px(15.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
        TypingInputBox,
    ));
}

fn update_ui(
    player_query: Query<&Ship, With<Player>>,
    wave_res: Res<crate::resources::Wave>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<HpText>, Without<ComboText>, Without<WaveText>, Without<WeaponText>)>,
    mut hp_query: Query<&mut Text, (With<HpText>, Without<ScoreText>, Without<ComboText>, Without<WaveText>, Without<WeaponText>)>,
    mut combo_query: Query<&mut Text, (With<ComboText>, Without<ScoreText>, Without<HpText>, Without<WaveText>, Without<WeaponText>)>,
    mut wave_query: Query<&mut Text, (With<WaveText>, Without<ScoreText>, Without<HpText>, Without<ComboText>, Without<WeaponText>)>,
    mut weapon_query: Query<&mut Text, (With<WeaponText>, Without<ScoreText>, Without<HpText>, Without<ComboText>, Without<WaveText>)>,
) {
    if let Ok(ship) = player_query.get_single() {
        if let Ok(mut text) = score_query.get_single_mut() {
            text.0 = format!("Score: {}", ship.score);
        }
        if let Ok(mut text) = hp_query.get_single_mut() {
            text.0 = format!("HP: {}", ship.hp);
        }
        if let Ok(mut text) = combo_query.get_single_mut() {
            text.0 = format!("Combo: {}", ship.combo);
        }
        if let Ok(mut text) = wave_query.get_single_mut() {
            text.0 = format!("Wave: {}", wave_res.current);
        }
        if let Ok(mut text) = weapon_query.get_single_mut() {
            text.0 = format!("Weapon: {:?}", ship.current_weapon);
        }
    }
}

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid_size = 20;
    let spacing = 50.0;
    let offset_x = -(grid_size as f32 * spacing) / 2.0;
    let offset_y = -(grid_size as f32 * spacing) / 2.0;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = offset_x + i as f32 * spacing;
            let y = offset_y + j as f32 * spacing;

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(2.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 1.0))),
                Transform::from_xyz(x, y, -10.0),
                GridPoint { original_pos: Vec3::new(x, y, -10.0) },
            ));
        }
    }
}

fn animate_background(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GridPoint)>,
) {
    for (mut transform, _point) in query.iter_mut() {
        transform.translation.y -= 50.0 * time.delta_secs();

        if transform.translation.y < -500.0 {
            transform.translation.y += 1000.0;
        }

        let y = transform.translation.y;
        let factor = 1.0 - ((y + 500.0) / 1000.0);
        
        transform.scale = Vec3::new(0.5 + factor * 1.5, 0.5 + factor * 1.5, 1.0);
    }
}

use bevy::input::keyboard::{KeyboardInput, Key};

fn update_typing_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut query: Query<(&mut Text, &mut TextFont), With<TypingInputBox>>,
) {
    for ev in key_evr.read() {
        if !ev.state.is_pressed() {
            continue;
        }
        
        match &ev.logical_key {
            Key::Character(s) => {
                for c in s.chars() {
                    if !c.is_control() {
                        typing_buffer.text.push(c);
                    }
                }
            }
            Key::Space => {
                typing_buffer.text.push(' ');
            }
            
            Key::Backspace => {
                typing_buffer.text.pop();
            }
            _ => {}
        }
    }
    
    if let Ok((mut text, mut font)) = query.get_single_mut() {
        text.0 = typing_buffer.text.clone();
        
        font.font_size = 35.0;
    }
}
