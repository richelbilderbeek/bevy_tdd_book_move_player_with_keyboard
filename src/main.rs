use crate::app::*;
use crate::game_parameters::*;
use bevy::prelude::*;
mod app;
mod game_parameters;
mod player;

fn main() {
    let mut app = create_app(create_default_game_parameters_with_player_velocity(
        Vec2::new(1.1, 2.2),
    ));
    let add_camera_fun = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fun);
    app.add_plugins(DefaultPlugins);
    app.run();
}
