use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_grid);
    }
}

#[derive(Component)]
struct GridLine;

#[derive(Component)]
struct GridPoint;

fn animate_grid(time: Res<Time>, mut gizmos: Gizmos) {
    let t = time.elapsed_secs() * 0.5;
    
    let color = Color::srgb(0.0, 0.8, 1.0).with_alpha(0.3);
    
    for i in 0..20 {
        let y_offset = (t * 100.0 + i as f32 * 50.0) % 1000.0 - 500.0;
        gizmos.line_2d(Vec2::new(-1000.0, y_offset), Vec2::new(1000.0, y_offset), color);
    }
    
    let vanishing_point = Vec2::new(0.0, 1000.0);
    
    for i in -10..=10 {
        let x_base = i as f32 * 100.0;
        gizmos.line_2d(Vec2::new(x_base, -500.0), vanishing_point, color);
    }
}
