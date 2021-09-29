// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*,};
use bevy::DefaultPlugins;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::window::WindowMode;

fn main() {
    let mut app = App::build();
    app
//      .insert_resource(Msaa { samples: 1 })
//      .insert_resource(WindowDescriptor {
//          width: 800.,
//          height: 600.,
//          title: "pewpewpew".to_string(), // ToDo
//          ..Default::default()
//      })
        .add_plugin(GamePlugin);

    app.run();
}


mod camera;
pub mod asset_loader;
pub mod game_controller;
pub mod player;
pub mod bullet;
mod field; 

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
           .add_state(AppState::Loading)

           .add_system_set(SystemSet::on_update(AppState::Loading)
                   .with_system(asset_loader::check_assets_ready.system())
           )
           .add_system_set(SystemSet::on_exit(AppState::Loading)
                   .with_system(fullscreen_app.system())
           )

           .add_plugin(field::FieldPlugin)
           .add_plugin(camera::CameraPlugin)
           .add_plugin(bullet::BulletPlugin)
           .init_resource::<asset_loader::AssetsLoading>()
           .add_system(debug_print_entity.system())
           .add_system(exit.system());
    }
}

fn exit(keys: Res<Input<KeyCode>>, mut exit: ResMut<Events<AppExit>>) {
    if keys.just_pressed(KeyCode::Q) || keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right, 
}

#[derive(PartialEq, Clone, Copy, Hash, std::cmp::Eq)]
pub enum Position {
    TopCenter,
    BottomCenter,
    Center,

    TopLeft,
    BottomLeft,
    Left,

    TopRight,
    BottomRight,
    Right,
}

pub fn fullscreen_app(
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    println!("Setting fullscreen...");
    window.set_maximized(true);
    window.set_mode(WindowMode::BorderlessFullscreen);
}

pub fn debug_print_entity(
    keyboard_input: Res<Input<KeyCode>>,
    entities: Query<&Transform, With<player::Moveable>>,
) {
    for transform in entities.iter() {
        if keyboard_input.pressed(KeyCode::Z) {
            let translation = transform.translation;
            let (rotation, axis) = transform.rotation.to_axis_angle();
            println!("x: {:?},", translation.x); 
            println!("y: {:?},", translation.y); 
            println!("z: {:?},", translation.z); 
            println!("rotation_x: {:?},", rotation.x); 
            println!("rotation_y: {:?},", rotation.y); 
            println!("rotation_z: {:?},", rotation.z); 
            println!("rotation_angle: {:?},", axis); 
        }
    }
}
