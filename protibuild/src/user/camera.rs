use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct CameraController {
    pub is_captured: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub initialized: bool,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            is_captured: false,
            pitch: 0.0,
            yaw: 0.0,
            speed: 5.0,
            sensitivity: 0.002,
            initialized: false,
        }
    }
}

#[derive(Resource)]
pub(crate) struct CameraTransformData {
    pub transform: Transform,
}

pub(crate) struct Camera;

impl Camera {
    pub(crate) fn init(app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::update);
    }

    pub(crate) fn setup(mut commands: Commands) {
        commands.insert_resource(CameraTransformData {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        });

        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            CameraController::default(),
        ));
    }

    pub(crate) fn update(
        camera_query: Query<&Transform, With<CameraController>>,
        mut camera_data: ResMut<CameraTransformData>,
    ) {
        if let Ok(camera_transform) = camera_query.single() {
            camera_data.transform = *camera_transform;
        }
    }
}
