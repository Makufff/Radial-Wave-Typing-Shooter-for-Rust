use bevy::prelude::*;
use crate::enemy::{Enemy, Word};
use crate::player::{Player, Ship};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (typing_system, weapon_switching, collision_system, draw_combat_effects));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Weapon {
    #[default]
    Blade,
    Laser,
}

fn weapon_switching(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Ship, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let mut ship = query.single_mut();
        ship.current_weapon = match ship.current_weapon {
            Weapon::Blade => Weapon::Laser,
            Weapon::Laser => Weapon::Blade,
        };
        println!("Switched to {:?}", ship.current_weapon);
    }
}

use bevy::input::keyboard::{KeyboardInput, Key};

#[derive(Component)]
struct LaserVisual {
    start: Vec3,
    end: Vec3,
    timer: Timer,
}

#[derive(Component)]
struct ExplosionVisual {
    position: Vec3,
    max_radius: f32,
    timer: Timer,
}

fn draw_combat_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut laser_query: Query<(Entity, &mut LaserVisual)>,
    mut explosion_query: Query<(Entity, &mut ExplosionVisual)>,
) {
    // Lasers
    for (entity, mut laser) in laser_query.iter_mut() {
        laser.timer.tick(time.delta());
        if laser.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            let alpha = laser.timer.remaining_secs() / laser.timer.duration().as_secs_f32();
            gizmos.line_2d(laser.start.truncate(), laser.end.truncate(), Color::srgb(0.0, 0.0, 1.0).with_alpha(alpha));
        }
    }

    // Explosions
    for (entity, mut explosion) in explosion_query.iter_mut() {
        explosion.timer.tick(time.delta());
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            let progress = 1.0 - (explosion.timer.remaining_secs() / explosion.timer.duration().as_secs_f32());
            let radius = explosion.max_radius * progress;
            let alpha = 1.0 - progress;
            gizmos.circle_2d(explosion.position.truncate(), radius, Color::srgb(0.0, 0.0, 1.0).with_alpha(alpha));
        }
    }
}

fn typing_system(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    mut enemy_query: Query<(Entity, &mut Word, &Children, &Transform), With<Enemy>>,
    mut text_color_query: Query<&mut TextColor>,
    mut text_transform_query: Query<&mut Transform, (With<Text2d>, Without<Enemy>, Without<Player>)>,
    mut player_query: Query<(Entity, &mut Ship, &mut Transform), (With<Player>, Without<Enemy>, Without<Text2d>)>,
    mut typing_buffer: ResMut<crate::ui::TypingBuffer>,
) {
    let (_player_entity, mut ship, mut player_transform) = player_query.single_mut();

    for ev in key_evr.read() {
        if !ev.state.is_pressed() {
            continue;
        }
        
        if let Key::Space = ev.logical_key {
            let typed_word = typing_buffer.text.trim().to_lowercase();
            
            if typed_word.is_empty() {
                continue;
            }
            
            let mut hit_any = false;
            let mut actions = Vec::new();

            for (entity, word, children, enemy_transform) in enemy_query.iter_mut() {
                if word.text.to_lowercase() == typed_word {
                    hit_any = true;
                    let children_vec: Vec<Entity> = children.iter().copied().collect();
                    let enemy_pos = enemy_transform.translation;
                    
                    actions.push((entity, children_vec, enemy_pos, ship.current_weapon));
                    break;
                }
            }
            
            for (entity, children_vec, enemy_pos, weapon) in actions {
                for &child in children_vec.iter() {
                    if let Ok(mut text_color) = text_color_query.get_mut(child) {
                        text_color.0 = Color::srgb(0.0, 1.0, 0.0);
                    }
                    if let Ok(mut text_transform) = text_transform_query.get_mut(child) {
                        text_transform.scale = Vec3::splat(0.08);
                    }
                }
                
                ship.score += 100 * (ship.combo + 1);
                ship.combo += 1;
                
                match weapon {
                    Weapon::Blade => {
                        player_transform.translation = enemy_pos;
                        println!("Blade Slide Kill!");
                        commands.entity(entity).despawn_recursive();
                    }
                    Weapon::Laser => {
                        commands.spawn(LaserVisual {
                            start: player_transform.translation,
                            end: enemy_pos,
                            timer: Timer::from_seconds(0.2, TimerMode::Once),
                        });
                        println!("Laser Shot!");
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
            
            if !hit_any {
                println!("Mistake!");
                ship.combo = 0;
                
                match ship.current_weapon {
                    Weapon::Blade => {
                        println!("Blade Parry!");
                    }
                    Weapon::Laser => {
                        commands.spawn(ExplosionVisual {
                            position: player_transform.translation + Vec3::new(0.0, 50.0, 0.0),
                            max_radius: 100.0,
                            timer: Timer::from_seconds(0.5, TimerMode::Once),
                        });
                        println!("Laser Explode!");
                    }
                }
            }
            
            typing_buffer.text.clear();
        }
    }
}

fn collision_system(
    mut commands: Commands,
    mut player_query: Query<(&mut Ship, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
) {
    if let Ok((mut ship, player_transform)) = player_query.get_single_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < 30.0 {
                ship.hp -= 1;
                ship.combo = 0;
                println!("Player Hit! HP: {}", ship.hp);
                commands.entity(enemy_entity).despawn_recursive();
                
                if ship.hp <= 0 {
                    println!("Game Over!");
                    next_state.set(crate::resources::GameState::GameOver);
                }
            }
        }
    }
}
