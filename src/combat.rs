use bevy::prelude::*;
use crate::enemy::{Enemy, Word};
use crate::player::{Player, Ship};
use crate::particles::{spawn_blade_trail, spawn_explosion, spawn_laser_hit, spawn_error_particles};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            weapon_switching,
            typing_system,
            collision_system,
        ).run_if(in_state(crate::resources::GameState::Running)));
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

fn typing_system(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    mut enemy_query: Query<(Entity, &mut Word, &mut crate::enemy::Health, &Children, &Transform), With<Enemy>>,
    mut text_color_query: Query<&mut TextColor>,
    mut text_transform_query: Query<&mut Transform, (With<Text2d>, Without<Enemy>, Without<Player>)>,
    mut player_query: Query<(Entity, &mut Ship, &mut Transform), (With<Player>, Without<Enemy>, Without<Text2d>)>,
    mut typing_buffer: ResMut<crate::ui::TypingBuffer>,
    difficulty: Res<crate::resources::Difficulty>,
) {
    let (_player_entity, mut ship, mut player_transform) = player_query.single_mut();

    for ev in key_evr.read() {
        if !ev.state.is_pressed() {
            continue;
        }
        
        if let Key::Enter | Key::Space = ev.logical_key {
            let typed_word = typing_buffer.text.trim();
            
            if typed_word.is_empty() {
                continue;
            }
            
            let mut hit_any = false;
            let mut actions = Vec::new();

            for (entity, word, health, children, enemy_transform) in enemy_query.iter_mut() {
                let matches = match *difficulty {
                    crate::resources::Difficulty::Easy => {
                        word.text.to_lowercase() == typed_word.to_lowercase()
                    }
                    crate::resources::Difficulty::Hard => {
                        word.text == typed_word
                    }
                };
                
                if matches {
                    hit_any = true;
                    let children_vec: Vec<Entity> = children.iter().copied().collect();
                    let enemy_pos = enemy_transform.translation;
                    let current_health = health.current;
                    
                    actions.push((entity, children_vec, enemy_pos, ship.current_weapon, current_health));
                    break;
                }
            }
            
            for (entity, children_vec, enemy_pos, weapon, current_health) in actions {
                match weapon {
                    Weapon::Blade => {
                        // Blade kills instantly
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
                        
                        // Spawn blade trail particles
                        let start_pos = player_transform.translation;
                        spawn_blade_trail(&mut commands, start_pos, enemy_pos);
                        
                        player_transform.translation = enemy_pos;
                        // Grant invulnerability to prevent collision damage during warp
                        ship.invulnerability_timer = Timer::from_seconds(0.15, TimerMode::Once);
                        
                        // Spawn explosion particles at enemy position
                        spawn_explosion(&mut commands, enemy_pos, Color::srgb(0.0, 1.0, 0.5), 20);
                        
                        println!("Blade Slide Kill!");
                        commands.entity(entity).despawn_recursive();
                    }
                    Weapon::Laser => {
                        // Get mutable health reference
                        if let Ok((_, _, mut health, _, _)) = enemy_query.get_mut(entity) {
                            health.current -= 1;
                            
                            if health.current <= 0 {
                                // Enemy destroyed
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
                                // Grant brief invulnerability after kill
                                ship.invulnerability_timer = Timer::from_seconds(0.15, TimerMode::Once);
                                
                                // Spawn explosion particles
                                spawn_explosion(&mut commands, enemy_pos, Color::srgb(0.0, 0.8, 1.0), 15);
                                
                                println!("Laser Kill!");
                                commands.entity(entity).despawn_recursive();
                            } else {
                                // Enemy damaged but not destroyed
                                for &child in children_vec.iter() {
                                    if let Ok(mut text_color) = text_color_query.get_mut(child) {
                                        text_color.0 = Color::srgb(1.0, 1.0, 0.0); // Yellow for damaged
                                    }
                                    if let Ok(mut text_transform) = text_transform_query.get_mut(child) {
                                        text_transform.scale = Vec3::splat(0.06);
                                    }
                                }
                                
                                // Spawn laser hit particles
                                spawn_laser_hit(&mut commands, enemy_pos);
                                
                                println!("Laser Hit! Enemy HP: {}", health.current);
                            }
                        }
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
                        // Spawn error particles
                        spawn_error_particles(&mut commands, player_transform.translation);
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
        // Skip collision damage if player is invulnerable
        if !ship.invulnerability_timer.finished() {
            return;
        }
        
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < 30.0 {
                ship.hp -= 1;
                ship.combo = 0;
                
                // Spawn collision explosion particles
                spawn_explosion(&mut commands, enemy_transform.translation, Color::srgb(1.0, 0.3, 0.0), 12);
                
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
