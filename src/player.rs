use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (player_movement, update_invulnerability).run_if(in_state(crate::resources::GameState::Running)));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub rotation_speed: f32,
    pub tilt_angle: f32,
    pub current_weapon: crate::combat::Weapon,
    pub score: u32,
    pub hp: i32,
    pub combo: u32,
    pub invulnerability_timer: Timer,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            speed: 300.0,
            rotation_speed: 10.0,
            tilt_angle: 0.0,
            current_weapon: crate::combat::Weapon::Blade,
            score: 0,
            hp: 3,
            combo: 0,
            invulnerability_timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 10.0).with_scale(Vec3::splat(30.0)),
        Player,
        Ship::default(),
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Ship), With<Player>>,
) {
    let (mut transform, ship) = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * ship.speed * time.delta_secs();

        let target_angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
        transform.rotation = Quat::from_rotation_z(target_angle);
        
        let wobble = (time.elapsed_secs() * 20.0).sin() * 0.1;
        transform.scale = Vec3::new(30.0 * (1.0 + wobble), 30.0, 1.0);
    } else {
        transform.scale = Vec3::splat(30.0);
    }
    transform.translation.z = 10.0;
}

fn update_invulnerability(
    time: Res<Time>,
    mut query: Query<&mut Ship, With<Player>>,
) {
    if let Ok(mut ship) = query.get_single_mut() {
        ship.invulnerability_timer.tick(time.delta());
    }
}
