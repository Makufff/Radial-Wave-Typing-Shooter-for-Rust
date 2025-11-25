use bevy::prelude::*;
use rand::Rng;
use crate::resources::{WordList, Wave};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
           .init_resource::<WordList>()
           .init_resource::<Wave>()
           .add_systems(Update, (spawn_enemies, enemy_movement, wave_progression, text_scale_recovery).run_if(in_state(crate::resources::GameState::Running)));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

#[derive(Component)]
pub struct Word {
    pub text: String,
    pub typed_index: usize,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    word_list: Res<WordList>,
    mut wave: ResMut<Wave>,
    difficulty: Res<crate::resources::Difficulty>,
) {
    if wave.enemies_remaining > 0 {
        if timer.0.tick(time.delta()).just_finished() {
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let radius = 500.0;
            let x = radius * angle.cos();
            let y = radius * angle.sin();

            let word_str = word_list.get_word(*difficulty);

            commands.spawn((
                Mesh2d(meshes.add(Triangle2d::default())),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(20.0)),
                Enemy { speed: 100.0 },
                Word {
                    text: word_str.clone(),
                    typed_index: 0,
                },
                Health {
                    current: 2,
                    max: 2,
                },
            )).with_children(|parent| {
                parent.spawn((
                    Text2d::new(word_str),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 1.5, 1.0),
                ));
            });
            
            wave.enemies_remaining -= 1;
        }
    }
}

fn wave_progression(
    mut wave: ResMut<Wave>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if wave.enemies_remaining == 0 && enemy_query.is_empty() {
        wave.current += 1;
        wave.enemies_remaining = wave.get_enemy_count();
        println!("Wave {} Started! Enemies: {}", wave.current, wave.enemies_remaining);
    }
}

fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Enemy)>,
    player_query: Query<&Transform, (With<crate::player::Player>, Without<Enemy>)>,
    mut time_virtual: ResMut<Time<Virtual>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        let mut closest_dist = f32::MAX;

        for (mut transform, enemy) in query.iter_mut() {
            let direction = (player_pos - transform.translation).normalize();
            transform.translation += direction * enemy.speed * time.delta_secs();

            let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(angle);
            
            let dist = transform.translation.distance(player_pos);
            if dist < closest_dist {
                closest_dist = dist;
            }
        }
        
        if closest_dist < 200.0 {
            time_virtual.set_relative_speed(0.5);
        } else {
            time_virtual.set_relative_speed(1.0);
        }
    }
}

fn text_scale_recovery(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, Without<Enemy>)>,
) {
    for mut transform in query.iter_mut() {
        let base_scale = Vec3::splat(0.05);
        transform.scale = transform.scale.lerp(base_scale, time.delta_secs() * 10.0);
    }
}
