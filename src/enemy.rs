use bevy::prelude::*;
use rand::Rng;
use crate::resources::{ContentManager, Wave};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
           .init_resource::<ContentManager>()
           .init_resource::<Wave>()
           .add_systems(Update, (
               spawn_enemies,
               enemy_movement,
               wave_progression,
               text_scale_recovery,
               shooting_enemy_fire_system,
               enemy_bullet_movement,
               enemy_bullet_collision,
           ).run_if(in_state(crate::resources::GameState::Running)));
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

#[derive(Component)]
pub struct EnemyText;

#[derive(Component)]
pub struct ShootingEnemy;

#[derive(Component)]
pub struct EnemyBullet {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct ShootTimer {
    pub timer: Timer,
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    content_manager: ResMut<ContentManager>,
    mut wave: ResMut<Wave>,
    difficulty: Res<crate::resources::Difficulty>,
    boss_query: Query<Entity, With<crate::boss::Boss>>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
) {
    if wave.current % 10 == 0 && wave.enemies_remaining > 0 {
        if boss_query.is_empty() {
            println!("Boss Wave {}! Entering warning screen...", wave.current);
            wave.enemies_remaining = 0;
            next_state.set(crate::resources::GameState::BossWarning);
        }
        return;
    }
    
    if wave.enemies_remaining > 0 {
        if timer.0.tick(time.delta()).just_finished() {
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let radius = 500.0;
            let x = radius * angle.cos();
            let y = radius * angle.sin();

            let word_str = content_manager.get_word(*difficulty);
            
            // 30% chance to spawn shooting enemy
            let is_shooting = rng.gen_bool(0.3);
            
            if is_shooting {
                // Spawn shooting enemy - slower, shoots bullets
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::from_size(Vec2::splat(1.0)))),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.6, 0.0))),
                    Transform::from_xyz(x, y, 10.0).with_scale(Vec3::splat(20.0)),
                    Enemy { speed: 60.0 },
                    ShootingEnemy,
                    ShootTimer {
                        timer: Timer::from_seconds(2.5, TimerMode::Repeating),
                    },
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
                        EnemyText,
                    ));
                });
            } else {
                // Spawn regular enemy
                commands.spawn((
                    Mesh2d(meshes.add(Triangle2d::default())),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                    Transform::from_xyz(x, y, 10.0).with_scale(Vec3::splat(20.0)),
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
                        EnemyText,
                    ));
                });
            }
            
            wave.enemies_remaining -= 1;
        }
    }
}

fn wave_progression(
    mut wave: ResMut<Wave>,
    enemy_query: Query<Entity, With<Enemy>>,
    boss_query: Query<Entity, With<crate::boss::Boss>>,
) {
    if wave.enemies_remaining == 0 && enemy_query.is_empty() && boss_query.is_empty() {
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
    mut query: Query<&mut Transform, With<EnemyText>>,
) {
    for mut transform in query.iter_mut() {
        let base_scale = Vec3::splat(0.05);
        transform.scale = transform.scale.lerp(base_scale, time.delta_secs() * 10.0);
    }
}

fn shooting_enemy_fire_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut ShootTimer), With<ShootingEnemy>>,
    player_query: Query<&Transform, With<crate::player::Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut shoot_timer) in query.iter_mut() {
            if shoot_timer.timer.tick(time.delta()).just_finished() {
                // Calculate direction to player
                let direction = (player_transform.translation - enemy_transform.translation).truncate().normalize();
                let velocity = direction * 150.0;
                
                // Spawn bullet
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(8.0))),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.8, 0.0))),
                    Transform::from_xyz(
                        enemy_transform.translation.x,
                        enemy_transform.translation.y,
                        9.0
                    ),
                    EnemyBullet { velocity },
                ));
            }
        }
    }
}

fn enemy_bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &EnemyBullet)>,
) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        transform.translation.x += bullet.velocity.x * time.delta_secs();
        transform.translation.y += bullet.velocity.y * time.delta_secs();
        
        // Despawn if too far from origin
        let distance = transform.translation.truncate().length();
        if distance > 800.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn enemy_bullet_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut crate::player::Ship), With<crate::player::Player>>,
    bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
) {
    if let Ok((player_transform, mut ship)) = player_query.get_single_mut() {
        if !ship.invulnerability_timer.finished() {
            return;
        }
        
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            let distance = player_transform.translation.distance(bullet_transform.translation);
            if distance < 30.0 {
                commands.entity(bullet_entity).despawn_recursive();
                
                ship.hp -= 1;
                ship.combo = 0;
                
                crate::particles::spawn_explosion(
                    &mut commands,
                    bullet_transform.translation,
                    Color::srgb(1.0, 0.5, 0.0),
                    8
                );
                
                println!("Hit by Enemy Bullet! HP: {}", ship.hp);
                
                if ship.hp <= 0 {
                    println!("Game Over!");
                    next_state.set(crate::resources::GameState::GameOver);
                }
                
                ship.invulnerability_timer = Timer::from_seconds(0.5, TimerMode::Once);
                
                break;
            }
        }
    }
}
