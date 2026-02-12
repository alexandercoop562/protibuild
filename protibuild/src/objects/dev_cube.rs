use bevy::prelude::*;

use crate::user::interaction::Movable;

#[derive(Component)]
pub(crate) struct Rotatable;

pub(crate) struct DevCube;

impl DevCube {
    pub(crate) fn init(app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::update);
    }

    pub(crate) fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Rotatable,
            Movable,
        ));
    }

    pub(crate) fn update(time: Res<Time>, mut query: Query<&mut Transform, With<Rotatable>>) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() * 1.0);
        }
    }
}
