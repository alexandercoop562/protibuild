use bevy::prelude::*;

use crate::user::camera::CameraTransformData;

pub struct GridPlugin;

#[derive(Component)]
pub(crate) struct GridLine;

#[derive(Component)]
pub(crate) struct AxisRay;

#[derive(Resource)]
pub(crate) struct GridConfig {
    spacing: f32,
    range: i32,
    grid_center: Vec3,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::update);
    }
}

impl GridPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let grid_material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: LinearRgba::WHITE,
            ..default()
        });

        let x_axis_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            emissive: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
            ..default()
        });
        let y_axis_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            emissive: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
            ..default()
        });
        let z_axis_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            emissive: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
            ..default()
        });

        let thickness = 0.04;
        let dash_length = 1.0;
        let spacing = 20.0;
        let range = 5;

        commands.insert_resource(GridConfig {
            spacing,
            range,
            grid_center: Vec3::ZERO,
        });

        let ray_length = 10000.0;
        let axis_thickness = 0.04 + 0.01;

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(ray_length, axis_thickness, axis_thickness))),
            MeshMaterial3d(x_axis_material),
            Transform::from_xyz(0.0, 0.0, 0.0),
            AxisRay,
        ));

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(axis_thickness, ray_length, axis_thickness))),
            MeshMaterial3d(y_axis_material),
            Transform::from_xyz(0.0, 0.0, 0.0),
            AxisRay,
        ));

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(axis_thickness, axis_thickness, ray_length))),
            MeshMaterial3d(z_axis_material),
            Transform::from_xyz(0.0, 0.0, 0.0),
            AxisRay,
        ));

        let mesh_x = meshes.add(Cuboid::new(dash_length, thickness, thickness));
        let mesh_y = meshes.add(Cuboid::new(thickness, dash_length, thickness));
        let mesh_z = meshes.add(Cuboid::new(thickness, thickness, dash_length));

        for j in -range..=range {
            for k in -range..=range {
                for i in -range..=range {
                    commands.spawn((
                        Mesh3d(mesh_x.clone()),
                        MeshMaterial3d(grid_material.clone()),
                        Transform::from_xyz(
                            i as f32 * spacing,
                            j as f32 * spacing,
                            k as f32 * spacing,
                        ),
                        GridLine,
                    ));
                }
            }
        }

        for i in -range..=range {
            for k in -range..=range {
                for j in -range..=range {
                    commands.spawn((
                        MeshMaterial3d(grid_material.clone()),
                        Mesh3d(mesh_y.clone()),
                        Transform::from_xyz(
                            i as f32 * spacing,
                            j as f32 * spacing,
                            k as f32 * spacing,
                        ),
                        GridLine,
                    ));
                }
            }
        }

        for i in -range..=range {
            for j in -range..=range {
                for k in -range..=range {
                    commands.spawn((
                        MeshMaterial3d(grid_material.clone()),
                        Mesh3d(mesh_z.clone()),
                        Transform::from_xyz(
                            i as f32 * spacing,
                            j as f32 * spacing,
                            k as f32 * spacing,
                        ),
                        GridLine,
                    ));
                }
            }
        }
    }

    fn update(
        mut grid_config: ResMut<GridConfig>,
        camera_data: Res<CameraTransformData>,
        mut grid_lines: Query<&mut Transform, (With<GridLine>, Without<AxisRay>)>,
    ) {
        let camera_pos = camera_data.transform.translation;
        let spacing = grid_config.spacing;

        let new_center_x = (camera_pos.x / spacing).round() * spacing;
        let new_center_y = (camera_pos.y / spacing).round() * spacing;
        let new_center_z = (camera_pos.z / spacing).round() * spacing;
        let new_center = Vec3::new(new_center_x, new_center_y, new_center_z);

        let threshold = spacing / 2.0;
        if (new_center - grid_config.grid_center).length() > threshold {
            grid_config.grid_center = new_center;

            let range = grid_config.range;
            let mut transforms: Vec<_> = grid_lines.iter_mut().collect();
            let mut index = 0;

            for j in -range..=range {
                for k in -range..=range {
                    for i in -range..=range {
                        if index < transforms.len() {
                            transforms[index].translation = Vec3::new(
                                new_center.x + i as f32 * spacing,
                                new_center.y + j as f32 * spacing,
                                new_center.z + k as f32 * spacing,
                            );
                        }
                        index += 1;
                    }
                }
            }

            for i in -range..=range {
                for k in -range..=range {
                    for j in -range..=range {
                        if index < transforms.len() {
                            transforms[index].translation = Vec3::new(
                                new_center.x + i as f32 * spacing,
                                new_center.y + j as f32 * spacing,
                                new_center.z + k as f32 * spacing,
                            );
                        }
                        index += 1;
                    }
                }
            }

            for i in -range..=range {
                for j in -range..=range {
                    for k in -range..=range {
                        if index < transforms.len() {
                            transforms[index].translation = Vec3::new(
                                new_center.x + i as f32 * spacing,
                                new_center.y + j as f32 * spacing,
                                new_center.z + k as f32 * spacing,
                            );
                        }
                        index += 1;
                    }
                }
            }
        }
    }
}
