use bevy::prelude::*;

use crate::user::interaction::Movable;

/// Marker component for entities that should rotate over time.
#[derive(Component)]
pub struct Rotatable;

/// Plugin for the development cube feature.
pub struct DevCubePlugin;

impl Plugin for DevCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rotatable);
    }
}

pub fn spawn_dev_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    _rotation: Quat,
    scale: Vec3,
) -> Entity {
    commands
        .spawn((
            Name::new("DevCube"),
            Mesh3d(meshes.add(Cuboid::new(1.0 * scale.x, 1.0 * scale.y, 1.0 * scale.z))),
            MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
            Transform::from_translation(position),
            Rotatable,
            Movable,
        ))
        .id()
}

fn update_rotatable(time: Res<Time>, mut query: Query<&mut Transform, With<Rotatable>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 1.0);
    }
}
