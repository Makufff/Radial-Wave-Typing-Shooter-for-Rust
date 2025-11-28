use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid_lines)
           .add_systems(Update, (animate_grid, update_map_size));
    }
}

#[derive(Component)]
struct GridLine {
    index: i32,
    is_horizontal: bool,
}

#[derive(Component)]
struct GridPoint;

#[derive(Component)]
pub enum Border {
    Top,
    Bottom,
    Left,
    Right,
}

fn setup_grid_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::srgb(0.0, 0.8, 1.0).with_alpha(0.3);
    let material_handle = materials.add(color);
    
    let h_mesh = meshes.add(Rectangle::new(2000.0, 2.0));
    
    for i in 0..20 {
        commands.spawn((
            Mesh2d(h_mesh.clone()),
            MeshMaterial2d(material_handle.clone()),
            Transform::from_xyz(0.0, 0.0, -20.0),
            GridLine { index: i, is_horizontal: true },
        ));
    }
    
    let v_mesh = meshes.add(Rectangle::new(2000.0, 2.0));
    
    for i in -10..=10 {
        commands.spawn((
            Mesh2d(v_mesh.clone()),
            MeshMaterial2d(material_handle.clone()),
            Transform::from_xyz(0.0, 0.0, -20.0),
            GridLine { index: i, is_horizontal: false },
        ));
    }
    
    // Add Map Borders
    let border_color = Color::srgb(0.0, 1.0, 1.0).with_alpha(0.8); // Cyan glowing borders
    let border_material = materials.add(border_color);
    
    // Use 1x1 rectangle as base for dynamic scaling
    let border_mesh = meshes.add(Rectangle::new(1.0, 1.0));
    
    // Top Border
    commands.spawn((
        Mesh2d(border_mesh.clone()),
        MeshMaterial2d(border_material.clone()),
        Transform::from_xyz(0.0, 0.0, -5.0),
        Border::Top,
    ));
    
    // Bottom Border
    commands.spawn((
        Mesh2d(border_mesh.clone()),
        MeshMaterial2d(border_material.clone()),
        Transform::from_xyz(0.0, 0.0, -5.0),
        Border::Bottom,
    ));
    
    // Left Border
    commands.spawn((
        Mesh2d(border_mesh.clone()),
        MeshMaterial2d(border_material.clone()),
        Transform::from_xyz(0.0, 0.0, -5.0),
        Border::Left,
    ));
    
    // Right Border
    commands.spawn((
        Mesh2d(border_mesh.clone()),
        MeshMaterial2d(border_material.clone()),
        Transform::from_xyz(0.0, 0.0, -5.0),
        Border::Right,
    ));
}

fn animate_grid(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GridLine)>,
) {
    let t = time.elapsed_secs() * 0.5;
    
    for (mut transform, line) in query.iter_mut() {
        if line.is_horizontal {
            let i = line.index;
            let y_offset = (t * 100.0 + i as f32 * 50.0) % 1000.0 - 500.0;
            transform.translation.y = y_offset;
            transform.translation.x = 0.0;
            transform.rotation = Quat::IDENTITY;
        } else {
            let i = line.index;
            let x_base = i as f32 * 100.0;
            let vanishing_point = Vec2::new(0.0, 1000.0);
            let start_point = Vec2::new(x_base, -500.0);
            
            let center = (start_point + vanishing_point) / 2.0;
            let diff = vanishing_point - start_point;
            let length = diff.length();
            let angle = diff.y.atan2(diff.x);
            
            transform.translation.x = center.x;
            transform.translation.y = center.y;
            transform.rotation = Quat::from_rotation_z(angle);
            transform.scale.x = length / 2000.0;
        }
    }
}

fn update_map_size(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut map_bounds: ResMut<crate::resources::MapBounds>,
    mut border_query: Query<(&mut Transform, &Border)>,
) {
    if let Ok(window) = window_query.get_single() {
        let width = window.width();
        let height = window.height();
        
        // Update MapBounds resource
        map_bounds.min_x = -width / 2.0;
        map_bounds.max_x = width / 2.0;
        map_bounds.min_y = -height / 2.0;
        map_bounds.max_y = height / 2.0;
        
        let border_thickness = 4.0;
        
        // Update visual borders
        for (mut transform, border) in border_query.iter_mut() {
            match border {
                Border::Top => {
                    transform.translation.y = height / 2.0;
                    transform.translation.x = 0.0;
                    transform.scale = Vec3::new(width + border_thickness, border_thickness, 1.0);
                }
                Border::Bottom => {
                    transform.translation.y = -height / 2.0;
                    transform.translation.x = 0.0;
                    transform.scale = Vec3::new(width + border_thickness, border_thickness, 1.0);
                }
                Border::Left => {
                    transform.translation.x = -width / 2.0;
                    transform.translation.y = 0.0;
                    transform.scale = Vec3::new(border_thickness, height + border_thickness, 1.0);
                }
                Border::Right => {
                    transform.translation.x = width / 2.0;
                    transform.translation.y = 0.0;
                    transform.scale = Vec3::new(border_thickness, height + border_thickness, 1.0);
                }
            }
        }
    }
}
