use bevy::prelude::*;

pub struct Bullet {
    direction: Vec3
}
pub struct BulletEvent {
    pub start: Vec3,
    pub direction: Vec3
}

static SPEED:f32 = 90.0;
static BULLET_DESPAWN_POINT:f32 = 500.0;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
       app.add_event::<BulletEvent>()
          .add_system_set(
              SystemSet::on_update(crate::AppState::InGame)
                  .with_system(update_bullets.system())
                  .with_system(handle_bullet_event.system())
          );
    }
}

fn update_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Bullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction * time.delta_seconds() * SPEED;

        if transform.translation.x > BULLET_DESPAWN_POINT {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_bullet_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut bullet_event_reader: EventReader<BulletEvent>,
) {
    for event in bullet_event_reader.iter() {
        commands.spawn()
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
                    transform: Transform::from_translation(event.start),
                    ..Default::default()
                })
                .insert(Bullet {
                    direction: event.direction
                });
    }
}
