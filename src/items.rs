use bevy::prelude::*;
use rand::Rng;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemSpawnTimer(Timer::from_seconds(17.0, TimerMode::Repeating)))
           .add_systems(Update, (
            health_item_movement,
            health_item_collection,
            spawn_periodic_items,
        ).run_if(in_state(crate::resources::GameState::Running)));
    }
}

#[derive(Resource)]
struct ItemSpawnTimer(Timer);

#[derive(Component)]
pub struct HealthItem {
    pub fall_speed: f32,
}

pub fn spawn_health_item(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(position.x, position.y, 9.0),
        HealthItem {
            fall_speed: 120.0,
        },
    )).with_children(|parent| {
        parent.spawn((
            Text2d::new("+1"),
            TextFont {
                font_size: 25.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(0.05)),
        ));
    });
}

fn health_item_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &HealthItem)>,
) {
    for (entity, mut transform, item) in query.iter_mut() {
        // Fall downward
        transform.translation.y -= item.fall_speed * time.delta_secs();
        
        // Despawn if off screen (below)
        if transform.translation.y < -500.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn health_item_collection(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut crate::player::Ship), With<crate::player::Player>>,
    item_query: Query<(Entity, &Transform), With<HealthItem>>,
) {
    if let Ok((player_transform, mut ship)) = player_query.get_single_mut() {
        for (item_entity, item_transform) in item_query.iter() {
            let distance = player_transform.translation.distance(item_transform.translation);
            if distance < 40.0 {
                // Pickup health item
                ship.hp += 1;
                
                // Visual feedback
                crate::particles::spawn_explosion(
                    &mut commands,
                    item_transform.translation,
                    Color::srgb(0.0, 1.0, 0.0),
                    12
                );
                
                println!("Health item collected! HP: {}", ship.hp);
                
                commands.entity(item_entity).despawn_recursive();
            }
        }
    }
}

fn spawn_periodic_items(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ItemSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-300.0..300.0);
        let y = 450.0;
        
        spawn_health_item(&mut commands, &mut meshes, &mut materials, Vec3::new(x, y, 9.0));
        println!("Periodic health item spawned at ({}, {})", x, y);
    }
}
