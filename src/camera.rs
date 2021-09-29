use bevy::prelude::*;
use crate::field::LevelReady;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
               SystemSet::on_update(crate::AppState::InGame)
                         .with_system(update_camera.system())
           );
    }
}

fn update_camera(
    mut cameras: Query<&mut Transform, Without<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // for debugging camera placement
    if keyboard_input.just_pressed(KeyCode::P) {
        for transform in cameras.iter_mut() {
            let translation = transform.translation;
            let (rotation, axis) = transform.rotation.to_axis_angle();
            println!("camera_x: {:?},", translation.x); 
            println!("camera_y: {:?},", translation.y); 
            println!("camera_z: {:?},", translation.z); 
            println!("camera_rotation_x: {:?},", rotation.x); 
            println!("camera_rotation_y: {:?},", rotation.y); 
            println!("camera_rotation_z: {:?},", rotation.z); 
            println!("camera_rotation_angle: {:?},", axis); 
        }
    }
}

pub struct MainCamera;

pub fn create_camera(
    mut commands: Commands,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    level_ready: Res<LevelReady>,
) {
    if !level_ready.0 {
        return; // level isn't loaded so we'll try again later
    }

    let mut transform = Transform::default();

    transform.translation = Vec3::new(-16.0, 5.0, 0.0);
    transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, -0.9905375, 0.0), std::f32::consts::PI / 2.0);

    if let Ok(mut camera_transform) = cameras.single_mut() {
        *camera_transform = transform;
    } else {
        println!("Creating camera!");

        commands
            .spawn_bundle(PerspectiveCameraBundle {
                transform, 
                ..Default::default()
            })
            .insert(MainCamera);
    }
}
