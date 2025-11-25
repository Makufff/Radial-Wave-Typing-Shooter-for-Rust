use bevy::prelude::*;
use rand::Rng;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_particles,
            render_particles,
        ).run_if(in_state(crate::resources::GameState::Running)));
    }
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
    pub lifetime: Timer,
    pub max_lifetime: f32,
    pub color: Color,
    pub size: f32,
    pub particle_type: ParticleType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ParticleType {
    Trail,      // Blade trail particles
    Explosion,  // Enemy death explosion
    Hit,        // Laser hit effect
    Error,      // Mistake particles
}

pub fn spawn_blade_trail(commands: &mut Commands, start: Vec3, end: Vec3) {
    let mut rng = rand::thread_rng();
    let direction = (end - start).normalize();
    let distance = start.distance(end);
    
    let particle_count = (distance / 20.0).max(5.0) as i32;
    
    for i in 0..particle_count {
        let t = i as f32 / particle_count as f32;
        let position = start.lerp(end, t);
        
        let offset = Vec3::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
            0.0,
        );
        
        commands.spawn((
            Transform::from_translation(position + offset),
            Particle {
                velocity: direction * rng.gen_range(50.0..150.0) + Vec3::new(
                    rng.gen_range(-50.0..50.0),
                    rng.gen_range(-50.0..50.0),
                    0.0,
                ),
                lifetime: Timer::from_seconds(rng.gen_range(0.2..0.5), TimerMode::Once),
                max_lifetime: 0.5,
                color: Color::srgb(0.0, 0.5, 1.0), // Blue trail
                size: rng.gen_range(2.0..5.0),
                particle_type: ParticleType::Trail,
            },
        ));
    }
}

pub fn spawn_explosion(commands: &mut Commands, position: Vec3, color: Color, count: i32) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(100.0..300.0);
        let velocity = Vec3::new(
            angle.cos() * speed,
            angle.sin() * speed,
            0.0,
        );
        
        commands.spawn((
            Transform::from_translation(position),
            Particle {
                velocity,
                lifetime: Timer::from_seconds(rng.gen_range(0.3..0.8), TimerMode::Once),
                max_lifetime: 0.8,
                color,
                size: rng.gen_range(3.0..8.0),
                particle_type: ParticleType::Explosion,
            },
        ));
    }
}

pub fn spawn_laser_hit(commands: &mut Commands, position: Vec3) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..8 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..150.0);
        let velocity = Vec3::new(
            angle.cos() * speed,
            angle.sin() * speed,
            0.0,
        );
        
        commands.spawn((
            Transform::from_translation(position),
            Particle {
                velocity,
                lifetime: Timer::from_seconds(rng.gen_range(0.2..0.4), TimerMode::Once),
                max_lifetime: 0.4,
                color: Color::srgb(0.0, 0.8, 1.0), // Cyan for laser
                size: rng.gen_range(2.0..4.0),
                particle_type: ParticleType::Hit,
            },
        ));
    }
}

pub fn spawn_error_particles(commands: &mut Commands, position: Vec3) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..12 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(80.0..200.0);
        let velocity = Vec3::new(
            angle.cos() * speed,
            angle.sin() * speed,
            0.0,
        );
        
        commands.spawn((
            Transform::from_translation(position + Vec3::new(0.0, 30.0, 0.0)),
            Particle {
                velocity,
                lifetime: Timer::from_seconds(rng.gen_range(0.3..0.6), TimerMode::Once),
                max_lifetime: 0.6,
                color: Color::srgb(1.0, 0.2, 0.2), // Red for errors
                size: rng.gen_range(3.0..6.0),
                particle_type: ParticleType::Error,
            },
        ));
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    for (entity, mut transform, mut particle) in query.iter_mut() {
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            // Update position based on velocity
            transform.translation += particle.velocity * time.delta_secs();
            
            // Apply drag/friction
            particle.velocity *= 0.95;
        }
    }
}

fn render_particles(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Particle)>,
) {
    for (transform, particle) in query.iter() {
        // Calculate alpha based on remaining lifetime
        let life_ratio = particle.lifetime.remaining_secs() / particle.max_lifetime;
        let alpha = life_ratio.clamp(0.0, 1.0);
        
        // Draw particle as a circle
        let color = particle.color.with_alpha(alpha);
        gizmos.circle_2d(
            transform.translation.truncate(),
            particle.size,
            color,
        );
    }
}
