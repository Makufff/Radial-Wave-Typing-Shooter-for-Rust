use bevy::prelude::*;
use crate::player::Player;
use crate::particles::spawn_explosion;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            boss_particle_system,
            boss_typing_system,
            boss_particle_movement,
            boss_collision_system,
            boss_health_bar_update,
        ).run_if(in_state(crate::resources::GameState::Running)))
        .add_systems(OnEnter(crate::resources::GameState::BossWarning), setup_boss_warning)
        .add_systems(Update, boss_warning_countdown.run_if(in_state(crate::resources::GameState::BossWarning)))
        .add_systems(OnExit(crate::resources::GameState::BossWarning), cleanup_boss_warning);
    }
}

#[derive(Component)]
pub struct Boss {
    pub health: usize,
    pub max_health: usize,
    pub particle_timer: Timer,
}

#[derive(Component)]
pub struct BossLine {
    pub lines: Vec<String>,
    pub current_line_index: usize,
}

#[derive(Component)]
pub struct BossParticle {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct BossHealthBar;

#[derive(Component)]
pub struct BossHealthFill;

#[derive(Component)]
pub struct BossWarningText;

#[derive(Resource)]
pub struct BossWarningTimer {
    pub timer: Timer,
    pub count: u32,
}

pub fn spawn_boss(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    content_manager: &crate::resources::ContentManager,
) {
    let lines = content_manager.get_current_lines();
    let health = lines.len();
    
    println!("Spawning Boss with {} HP (lines)", health);
    
    commands.spawn((
        Node {
            width: Val::Px(600.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-300.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        BorderColor(Color::WHITE),
        BossHealthBar,
    )).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
            BossHealthFill,
        ));
        
        parent.spawn((
            Text::new(format!("BOSS HP: {}/{}", health, health)),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(5.0),
                ..default()
            },
            BossHealthBar,
        ));
    });
    
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.0, 0.8))),
        Transform::from_xyz(0.0, 0.0, 10.0),
        Boss {
            health,
            max_health: health,
            particle_timer: Timer::from_seconds(3.5, TimerMode::Repeating),
        },
        BossLine {
            lines: lines.clone(),
            current_line_index: 0,
        },
    )).with_children(|parent| {
        parent.spawn((
            Text2d::new(&lines[0]),
            TextFont {
                font_size: (50.0 * (20.0 / lines[0].len().max(1) as f32).min(1.0)).clamp(20.0, 50.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            Transform::from_xyz(0.0, 100.0, 20.0),
        ));
    });
}

fn boss_particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut boss_query: Query<(&mut Boss, &Transform)>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use rand::Rng;
    
    for (mut boss, transform) in boss_query.iter_mut() {
        if boss.particle_timer.tick(time.delta()).just_finished() {
            let speed = 90.0;
            let mut rng = rand::thread_rng();
            
            let pattern = rng.gen_range(0..8);
            
            match pattern {
                0 => {
                    let particle_count = 12;
                    for i in 0..particle_count {
                        let angle = (i as f32 / particle_count as f32) * std::f32::consts::TAU;
                        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
                        
                        commands.spawn((
                            Mesh2d(meshes.add(Circle::new(12.0))),
                            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                            Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0),
                            BossParticle { velocity },
                        ));
                    }
                }
                1 => {
                    let particle_count = 6;
                    let offset_angle = time.elapsed_secs() * 2.0;
                    for i in 0..particle_count {
                        let angle = (i as f32 / particle_count as f32) * std::f32::consts::TAU + offset_angle;
                        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
                        
                        commands.spawn((
                            Mesh2d(meshes.add(Circle::new(12.0))),
                            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.2, 0.0))),
                            Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0),
                            BossParticle { velocity },
                        ));
                    }
                }
                2 => {
                    for i in 0..4 {
                        let angle = i as f32 * std::f32::consts::FRAC_PI_2;
                        for j in 0..3 {
                            let velocity = Vec2::new(angle.cos(), angle.sin()) * speed * (1.0 + j as f32 * 0.3);
                            
                            commands.spawn((
                                Mesh2d(meshes.add(Circle::new(15.0))),
                                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.3))),
                                Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0),
                                BossParticle { velocity },
                            ));
                        }
                    }
                }
                3 => {
                    if let Ok(player_transform) = player_query.get_single() {
                        let diff = player_transform.translation - transform.translation;
                        let angle_to_player = diff.y.atan2(diff.x);
                        
                        for i in -2..=2 {
                            let angle = angle_to_player + (i as f32 * 0.15);
                            let velocity = Vec2::new(angle.cos(), angle.sin()) * speed * 1.5;
                            
                            commands.spawn((
                                Mesh2d(meshes.add(Circle::new(10.0))),
                                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
                                Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0),
                                BossParticle { velocity },
                            ));
                        }
                    }
                }
                4 => {
                    let points = 12;
                    let offset = time.elapsed_secs() * 3.0;
                    for i in 0..points {
                        let angle = (i as f32 / points as f32) * std::f32::consts::TAU * 2.0 + offset;
                        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed * 0.9;
                        
                        commands.spawn((
                            Mesh2d(meshes.add(Triangle2d::default())),
                            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
                            Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0).with_scale(Vec3::splat(20.0)),
                            BossParticle { velocity },
                        ));
                    }
                }
                5 => {
                    let wave_count = 3;
                    for wave in 0..wave_count {
                        let base_angle = (wave as f32 / wave_count as f32) * std::f32::consts::TAU;
                        for i in 0..5 {
                            let spread = (i as f32 - 2.0) * 0.3;
                            let angle = base_angle + spread;
                            let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
                            
                            commands.spawn((
                                Mesh2d(meshes.add(Circle::new(10.0))),
                                MeshMaterial2d(materials.add(Color::srgb(0.0, 0.8, 1.0))),
                                Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0),
                                BossParticle { velocity },
                            ));
                        }
                    }
                }
                6 => {
                    for _ in 0..12 {
                        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                        let speed_mult = rng.gen_range(0.7..1.5);
                        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed * speed_mult;
                        
                        commands.spawn((
                            Mesh2d(meshes.add(Circle::new(8.0))),
                            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
                            Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0),
                            BossParticle { velocity },
                        ));
                    }
                }
                _ => {
                    let minion_count = 4;
                    for i in 0..minion_count {
                        let angle = (i as f32 / minion_count as f32) * std::f32::consts::TAU + rng.gen_range(0.0..0.5);
                        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed * 0.5;
                        
                        commands.spawn((
                            Mesh2d(meshes.add(Circle::new(20.0))),
                            MeshMaterial2d(materials.add(Color::srgb(0.8, 0.0, 0.8))),
                            Transform::from_xyz(transform.translation.x, transform.translation.y, 9.0),
                            BossParticle { velocity },
                        ));
                    }
                }
            }
        }
    }
}

fn boss_particle_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &BossParticle)>,
) {
    for (entity, mut transform, particle) in particle_query.iter_mut() {
        transform.translation.x += particle.velocity.x * time.delta_secs();
        transform.translation.y += particle.velocity.y * time.delta_secs();
        
        let distance = transform.translation.truncate().length();
        if distance > 800.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn boss_collision_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut crate::player::Ship), With<Player>>,
    particle_query: Query<(Entity, &Transform), With<BossParticle>>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
) {
    if let Ok((player_transform, mut ship)) = player_query.get_single_mut() {
        if !ship.invulnerability_timer.finished() {
            return;
        }
        
        for (particle_entity, particle_transform) in particle_query.iter() {
            let distance = player_transform.translation.distance(particle_transform.translation);
            if distance < 30.0 {
                commands.entity(particle_entity).despawn_recursive();
                
                ship.hp -= 1;
                ship.combo = 0;
                
                spawn_explosion(&mut commands, particle_transform.translation, Color::srgb(1.0, 0.0, 0.0), 10);
                
                println!("Hit by Boss Particle! HP: {}", ship.hp);
                
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

fn boss_health_bar_update(
    boss_query: Query<&Boss>,
    mut health_bar_text_query: Query<&mut Text, With<BossHealthBar>>,
    mut health_fill_query: Query<&mut Node, With<BossHealthFill>>,
) {
    if let Ok(boss) = boss_query.get_single() {
        for mut text in health_bar_text_query.iter_mut() {
            **text = format!("BOSS HP: {}/{}", boss.health, boss.max_health);
        }
        
        if let Ok(mut node) = health_fill_query.get_single_mut() {
            let percent = (boss.health as f32 / boss.max_health as f32) * 100.0;
            node.width = Val::Percent(percent);
        }
    }
}

pub fn boss_typing_system(
    mut commands: Commands,
    mut key_evr: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut boss_query: Query<(Entity, &mut Boss, &mut BossLine, &Children, &Transform)>,
    mut text_query: Query<(&mut Text2d, &mut TextFont)>,
    mut typing_buffer: ResMut<crate::ui::TypingBuffer>,
    mut content_manager: ResMut<crate::resources::ContentManager>,
    mut wave: ResMut<crate::resources::Wave>,
    difficulty: Res<crate::resources::Difficulty>,
    health_bar_query: Query<Entity, With<BossHealthBar>>,
    mut player_query: Query<&mut crate::player::Ship, With<Player>>,
) {
    use bevy::input::keyboard::Key;
    
    for ev in key_evr.read() {
        if !ev.state.is_pressed() {
            continue;
        }
        
        if let Key::Enter = ev.logical_key {
            let typed_text = typing_buffer.text.trim();
            
            if typed_text.is_empty() {
                continue;
            }
            
            if let Ok((boss_entity, mut boss, mut boss_line, children, boss_transform)) = boss_query.get_single_mut() {
                let current_line = boss_line.lines[boss_line.current_line_index].clone();
                
                let matches = match *difficulty {
                    crate::resources::Difficulty::Easy => {
                        current_line.to_lowercase() == typed_text.to_lowercase()
                    }
                    crate::resources::Difficulty::Hard => {
                        current_line == typed_text
                    }
                };
                
                if matches {
                    println!("Boss line typed correctly!");
                    boss.health -= 1;
                    boss_line.current_line_index += 1;
                    
                    if let Ok(mut ship) = player_query.get_single_mut() {
                        let line_len = current_line.len() as u32;
                        ship.score += line_len * 100 * (ship.combo + 1);
                        ship.combo += 1;
                    }
                    
                    let children_vec: Vec<Entity> = children.iter().copied().collect();
                    if !children_vec.is_empty() {
                        if let Ok((mut text, mut font)) = text_query.get_mut(children_vec[0]) {
                            if boss_line.current_line_index < boss_line.lines.len() {
                                **text = boss_line.lines[boss_line.current_line_index].clone();
                                let line_len = text.len().max(1) as f32;
                                font.font_size = (50.0 * (20.0 / line_len).min(1.0)).clamp(20.0, 50.0);
                            } else {
                                **text = "DEFEATED!".to_string();
                            }
                        }
                    }
                    
                    if boss.health == 0 {
                        println!("Boss Defeated!");
                        
                        spawn_explosion(&mut commands, boss_transform.translation, Color::srgb(0.8, 0.0, 0.8), 30);
                        
                        content_manager.next_paragraph();
                        
                        commands.entity(boss_entity).despawn_recursive();
                        
                        for entity in health_bar_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        
                        wave.enemies_remaining = 0;
                        println!("Boss defeated! Wave progression will continue...");
                    }
                } else {
                    println!("Wrong line typed!");
                }
                
                typing_buffer.text.clear();
            }
        }
    }
}

fn setup_boss_warning(
    mut commands: Commands,
) {
    commands.insert_resource(BossWarningTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        count: 3,
    });
    
    commands.spawn((
        Text2d::new("BOSS WARNING"),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)),
        Transform::from_xyz(0.0, 100.0, 10.0),
        BossWarningText,
    ));
    
    commands.spawn((
        Text2d::new("3"),
        TextFont {
            font_size: 120.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Transform::from_xyz(0.0, -50.0, 10.0),
        BossWarningText,
    ));
}

fn boss_warning_countdown(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<BossWarningTimer>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
    mut text_query: Query<&mut Text2d, With<BossWarningText>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    content_manager: Res<crate::resources::ContentManager>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        if timer.count > 0 {
            timer.count -= 1;
            
            if timer.count > 0 {
                for mut text in text_query.iter_mut() {
                    if text.0.contains("⚠️") {
                        continue;
                    }
                    **text = timer.count.to_string();
                }
            } else {
                spawn_boss(&mut commands, &mut meshes, &mut materials, &content_manager);
                next_state.set(crate::resources::GameState::Running);
            }
        }
    }
}

fn cleanup_boss_warning(
    mut commands: Commands,
    warning_query: Query<Entity, With<BossWarningText>>,
) {
    for entity in warning_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<BossWarningTimer>();
}
