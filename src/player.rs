use bevy::prelude::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::{Position, Direction, game_controller, bullet};

static SPACE:f32 = 3.0;
static CENTER:f32 = 5.0;

lazy_static!{
    static ref POSITION_MAP: HashMap<Position, Vec2> = [
        (Position::TopCenter, Vec2::new(CENTER, SPACE)),
        (Position::BottomCenter ,Vec2::new(CENTER, -SPACE)),
        (Position::Center, Vec2::new(CENTER, CENTER)),

        (Position::TopLeft, Vec2::new(-SPACE, SPACE)),
        (Position::BottomLeft, Vec2::new(-SPACE, -SPACE)),
        (Position::Left, Vec2::new(-SPACE, CENTER)),

        (Position::TopRight, Vec2::new(CENTER, CENTER)),
        (Position::BottomRight, Vec2::new(SPACE, -SPACE)),
        (Position::Right, Vec2::new(SPACE, CENTER)),
    ].iter().copied().collect();
}

pub struct Player;

pub struct Moveable {
    position: Position, 
    movement: Movement,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Movement {
    Stopped,
    Queued(Direction),
    Moving(MovementInfo),
}

#[derive(PartialEq, Clone, Copy)]
pub struct MovementInfo {
    current_movement_time: f32,
    end_movement_time: f32,

    start_translation: Vec2,
    end_translation: Vec2,

    start_rotation: Quat,
    end_rotation: Quat,

    start_position: Position,
    end_position: Position,
}

pub fn spawn_player(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {
    commands.spawn_bundle(PbrBundle {
                transform: Transform::from_translation(Vec3::new(0.0, CENTER, 0.0)),
                ..Default::default()
            })
            .insert(Player { })
            .insert(Moveable {
                position: Position::Center,
                movement: Movement::Stopped,
            })
            .with_children(|parent|  {
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    ..Default::default()
                });
            })
            .id()
}

pub fn update_moveables(
    mut moveable: Query<(&mut Moveable, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut moveable, mut transform) in moveable.iter_mut() {
        moveable.movement =
        match moveable.movement {
            Movement::Moving(mut movement) => {
                movement.current_movement_time += time.delta_seconds();

                let new_translation = movement.start_translation.lerp(movement.end_translation, 
                                                                  movement.current_movement_time / movement.end_movement_time);
                let new_rotation = movement.start_rotation.lerp(movement.end_rotation, 
                                                                movement.current_movement_time / movement.end_movement_time);
                if movement.current_movement_time >= movement.end_movement_time {
                    transform.translation.y = movement.end_translation.y;
                    transform.translation.z = movement.end_translation.x;
                    transform.rotation = movement.end_rotation;
                    moveable.position = movement.end_position;

                    Movement::Stopped
                } else {
                    if !new_translation.is_nan() {
                        let current = Vec2::new(transform.translation.z, transform.translation.y); 
                        if current.distance(movement.end_translation) < current.distance(new_translation) {
                            transform.translation.y = movement.end_translation.y;
                            transform.translation.z = movement.end_translation.x;
                            movement.current_movement_time = movement.end_movement_time;
                        } else {
                            transform.translation.y = new_translation.y;
                            transform.translation.z = new_translation.x;
                        }
                    }

                    // update position if we're close enough
                    if moveable.position != movement.end_position && 
                        transform.translation.as_i32().y == movement.end_translation.as_i32().y &&
                        transform.translation.as_i32().z == movement.end_translation.as_i32().x {
                        moveable.position = movement.end_position;
                    }

                    if !new_rotation.is_nan() {
                        if transform.rotation.angle_between(movement.end_rotation) < transform.rotation.angle_between(new_rotation) {
                            transform.rotation = movement.end_rotation;
                        } else {
                            transform.rotation = new_rotation;
                        }
                    }

                    Movement::Moving(movement)
                }
            },
            Movement::Queued(direction) => {
                let mut new_movement = 
                    MovementInfo {
                        current_movement_time: 0.0,
                        end_movement_time: 0.10,

                        start_translation: Vec2::new(transform.translation.z, transform.translation.y),
                        end_translation: Vec2::new(transform.translation.z, transform.translation.y),

                        start_rotation: transform.rotation,
                        end_rotation: transform.rotation,

                        start_position: moveable.position,
                        end_position: moveable.position,
                    };

                match direction {
                    Direction::Up => {
                        new_movement.end_translation += Vec2::new(0.0, SPACE);
                        new_movement.end_position = 
                            match moveable.position {
                                Position::Center => Position::TopCenter,
                                Position::Left => Position::TopLeft,
                                Position::Right => Position::TopRight,
                                Position::BottomCenter => Position::Center,
                                Position::BottomLeft => Position::Left,
                                Position::BottomRight => Position::Right,
                                _ => new_movement.start_position
                            };
                    },
                    Direction::Down => {
                        new_movement.end_translation += Vec2::new(0.0, -SPACE);
                        new_movement.end_position = 
                            match moveable.position {
                                Position::TopCenter => Position::Center,
                                Position::TopLeft => Position::Left,
                                Position::TopRight => Position::Right,
                                Position::Center => Position::BottomCenter,
                                Position::Left => Position::BottomLeft,
                                Position::Right => Position::BottomRight,
                                _ => new_movement.start_position
                            };
                    },
                    Direction::Left => {
                        new_movement.end_translation += Vec2::new(-SPACE, 0.0);
                        new_movement.end_position = 
                            match moveable.position {
                                Position::Center => Position::Left,
                                Position::Right => Position::Center,
                                Position::BottomCenter => Position::BottomLeft,
                                Position::BottomRight => Position::BottomCenter,
                                Position::TopCenter => Position::TopLeft,
                                Position::TopRight => Position::TopCenter,
                                _ => new_movement.start_position
                            };
                    },
                    Direction::Right => {
                        new_movement.end_translation += Vec2::new(SPACE, 0.0);
                        new_movement.end_position = 
                            match moveable.position {
                                Position::Center => Position::Right,
                                Position::Left => Position::Center,
                                Position::BottomCenter => Position::BottomRight,
                                Position::BottomLeft => Position::BottomCenter,
                                Position::TopCenter => Position::TopRight,
                                Position::TopLeft => Position::TopCenter,
                                _ => new_movement.start_position
                            };
                    }
                }

                if new_movement.end_position != new_movement.start_position {
                    Movement::Moving(new_movement)
                } else {
                    Movement::Stopped
                }
            },
            Movement::Stopped => Movement::Stopped
        };
    }
}

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>, 
    mut player: Query<(&mut Moveable, &Transform), With<Player>>,
    mut action_buffer: Local<Option::<u128>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    gamepad: Option<Res<game_controller::GameController>>,
    mut bullet_event_writer: EventWriter<bullet::BulletEvent>,
) {
    let time_buffer = 100;

    let time_since_startup = time.time_since_startup().as_millis();
    if let Some(time_since_action) = *action_buffer {
        if time_since_startup - time_since_action > time_buffer {
            *action_buffer = None;
        }
    }

    let pressed_buttons = game_controller::get_pressed_buttons(&axes, &buttons, gamepad);
    for (mut player, transform) in player.iter_mut() {
        if (keyboard_input.pressed(KeyCode::Space) 
        || keyboard_input.pressed(KeyCode::Return) 
        || keyboard_input.pressed(KeyCode::J) 
        || pressed_buttons.contains(&game_controller::GameButton::Action))
        && action_buffer.is_none() {
            *action_buffer = Some(time.time_since_startup().as_millis());
            bullet_event_writer.send(bullet::BulletEvent {
                start: transform.translation,
                direction: Vec3::new(1.0, 0.0, 0.0)
            });
        }

        let mut move_dir = None;
        if keyboard_input.pressed(KeyCode::W) 
         || keyboard_input.pressed(KeyCode::Up) 
         || pressed_buttons.contains(&game_controller::GameButton::Up) {
            move_dir = Some(Direction::Up); 
        }
        if keyboard_input.pressed(KeyCode::S) 
           || keyboard_input.pressed(KeyCode::Down) 
           || pressed_buttons.contains(&game_controller::GameButton::Down) {
            move_dir = Some(Direction::Down); 
        }
        if keyboard_input.pressed(KeyCode::A) 
           || keyboard_input.pressed(KeyCode::Left) 
           || pressed_buttons.contains(&game_controller::GameButton::Left) {
            move_dir = Some(Direction::Left); 
        }
        if keyboard_input.pressed(KeyCode::D) 
           || keyboard_input.pressed(KeyCode::Right) 
           || pressed_buttons.contains(&game_controller::GameButton::Right) {
            move_dir = Some(Direction::Right); 
        }

        if player.movement == Movement::Stopped {
            if let Some(move_dir) = move_dir {
                player.movement = Movement::Queued(move_dir);
            }
        }
    }
}
