use bevy::prelude::*;

mod player;
mod enemy;
mod combat;
mod ui;
mod resources;
mod background;
mod game_over;
mod menu;
mod pause;
mod particles;
mod boss;
mod items;

use player::PlayerPlugin;
use enemy::EnemyPlugin;
use combat::CombatPlugin;
use ui::GameUiPlugin;
use resources::GameState;
use boss::BossPlugin;
use items::ItemsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((PlayerPlugin, EnemyPlugin, CombatPlugin, GameUiPlugin, BossPlugin, ItemsPlugin, crate::background::BackgroundPlugin, crate::game_over::GameOverPlugin, crate::menu::MenuPlugin, crate::pause::PausePlugin, crate::particles::ParticlePlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
