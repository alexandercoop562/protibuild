use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use super::camera::{CameraController, CameraTransformData};

#[derive(Component)]
pub(crate) struct Movable;

#[derive(Component)]
pub(crate) struct HeldMovable {
    distance: f32,
}

#[derive(Resource)]
pub(crate) struct HoveredMovable {
    pub entity: Option<Entity>,
}

#[derive(Resource)]
pub(crate) struct LastCameraPosition {
    position: Vec3,
    frame_count: u32,
}

fn ray_box_intersection(
    ray_origin: Vec3,
    ray_dir: Vec3,
    box_center: Vec3,
    half_extents: Vec3,
) -> Option<f32> {
    let inv_dir = Vec3::new(1.0 / ray_dir.x, 1.0 / ray_dir.y, 1.0 / ray_dir.z);

    let t1 = (box_center - half_extents - ray_origin) * inv_dir;
    let t2 = (box_center + half_extents - ray_origin) * inv_dir;

    let tmin = t1.min(t2);
    let tmax = t1.max(t2);

    let t0 = tmin.x.max(tmin.y).max(tmin.z);
    let t1 = tmax.x.min(tmax.y).min(tmax.z);

    if t0 > t1 {
        return None;
    }

    if t0 > 0.0 {
        Some(t0)
    } else if t1 > 0.0 {
        Some(t1)
    } else {
        None
    }
}

pub(crate) struct Interaction;

impl Interaction {
    pub(crate) fn init(app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::handle_input)
            .add_systems(Update, Self::update_hovered_object)
            .add_systems(Update, Self::camera_controls)
            .add_systems(Update, Self::handle_grab)
            .add_systems(
                Update,
                Self::update_held_objects.run_if(Self::any_object_held),
            )
            .add_systems(Update, Self::handle_scroll_adjust_distance)
            .add_systems(Last, Self::draw_highlight);
    }

    pub(crate) fn setup(mut commands: Commands) {
        commands.insert_resource(HoveredMovable { entity: None });
        commands.insert_resource(LastCameraPosition {
            position: Vec3::ZERO,
            frame_count: 0,
        });
    }

    fn any_object_held(held_query: Query<&HeldMovable>) -> bool {
        !held_query.is_empty()
    }

    pub(crate) fn handle_input(
        mut cursor_options_query: Query<&mut CursorOptions>,
        mouse_button: Res<ButtonInput<MouseButton>>,
        keyboard: Res<ButtonInput<KeyCode>>,
        mut controller_query: Query<&mut CameraController>,
    ) {
        let Ok(mut controller) = controller_query.single_mut() else {
            return;
        };
        let Ok(mut cursor_options) = cursor_options_query.single_mut() else {
            return;
        };

        if mouse_button.just_pressed(MouseButton::Left) && !controller.is_captured {
            cursor_options.grab_mode = CursorGrabMode::Locked;
            cursor_options.visible = false;
            controller.is_captured = true;
        }

        if keyboard.just_pressed(KeyCode::Escape) && controller.is_captured {
            cursor_options.grab_mode = CursorGrabMode::None;
            cursor_options.visible = true;
            controller.is_captured = false;
        }
    }

    pub(crate) fn update_hovered_object(
        camera_data: Res<CameraTransformData>,
        mut hovered: ResMut<HoveredMovable>,
        movable_query: Query<(Entity, &Transform), With<Movable>>,
        mut last_cam: ResMut<LastCameraPosition>,
    ) {
        // Throttle: only run every 3rd frame (20 times/sec instead of 60)
        last_cam.frame_count += 1;
        if !last_cam.frame_count.is_multiple_of(3) {
            return;
        }

        let current_pos = camera_data.transform.translation;
        let moved = (current_pos - last_cam.position).length_squared() > 0.001;

        if !moved {
            return;
        }

        last_cam.position = current_pos;

        let ray_origin = camera_data.transform.translation;
        let ray_dir = camera_data.transform.forward().as_vec3();

        let mut closest_entity = None;
        let mut closest_distance = f32::MAX;

        for (entity, transform) in movable_query.iter() {
            if let Some(distance) =
                ray_box_intersection(ray_origin, ray_dir, transform.translation, Vec3::splat(0.5))
                && distance < closest_distance
                && distance > 0.0
            {
                closest_distance = distance;
                closest_entity = Some(entity);
            }
        }

        hovered.entity = closest_entity;
    }

    pub(crate) fn draw_highlight(
        mut gizmos: Gizmos,
        hovered: Res<HoveredMovable>,
        movable_query: Query<&Transform, With<Movable>>,
        held_query: Query<(Entity, &Transform), With<HeldMovable>>,
    ) {
        match held_query.single() {
            Ok((_, transform)) => {
                gizmos.cube(*transform, Color::WHITE);
            }
            Err(_) => {
                if let Some(entity) = hovered.entity
                    && let Ok(transform) = movable_query.get(entity)
                {
                    gizmos.cube(*transform, Color::WHITE);
                }
            }
        }
    }

    pub(crate) fn handle_grab(
        mut commands: Commands,
        mouse_button: Res<ButtonInput<MouseButton>>,
        camera_data: Res<CameraTransformData>,
        controller_query: Query<&CameraController>,
        hovered: Res<HoveredMovable>,
        movable_query: Query<(Entity, &Transform), With<Movable>>,
        held_query: Query<(Entity, &HeldMovable)>,
    ) {
        let Ok(controller) = controller_query.single() else {
            return;
        };

        if !controller.is_captured {
            return;
        }

        let currently_held = held_query.iter().next();

        if mouse_button.just_released(MouseButton::Left) {
            if let Some((held_entity, _)) = currently_held {
                commands.entity(held_entity).remove::<HeldMovable>();
            }
            return;
        }

        if mouse_button.pressed(MouseButton::Left)
            && currently_held.is_none()
            && let Some(entity) = hovered.entity
        {
            let ray_origin = camera_data.transform.translation;
            let ray_dir = camera_data.transform.forward().as_vec3();

            if let Ok((_, transform)) = movable_query.get(entity)
                && let Some(distance) = ray_box_intersection(
                    ray_origin,
                    ray_dir,
                    transform.translation,
                    Vec3::splat(0.5),
                )
            {
                commands.entity(entity).insert(HeldMovable { distance });
            }
        }
    }

    pub(crate) fn update_held_objects(
        camera_data: Res<CameraTransformData>,
        mut held_query: Query<(&HeldMovable, &mut Transform)>,
    ) {
        let forward = camera_data.transform.forward().as_vec3();
        let cam_pos = camera_data.transform.translation;

        for (held, mut transform) in held_query.iter_mut() {
            let target_pos = cam_pos + forward * held.distance;
            transform.translation = transform.translation.lerp(target_pos, 0.3);
        }
    }

    pub(crate) fn camera_controls(
        mut camera_query: Query<(&mut Transform, &mut CameraController)>,
        keyboard: Res<ButtonInput<KeyCode>>,
        mut mouse_motion: MessageReader<MouseMotion>,
        time: Res<Time>,
    ) {
        let Ok((mut transform, mut controller)) = camera_query.single_mut() else {
            return;
        };

        if !controller.is_captured {
            return;
        }

        if !controller.initialized {
            let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
            controller.initialized = true;
        }

        let mut mouse_delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            mouse_delta += event.delta;
        }

        if mouse_delta != Vec2::ZERO {
            controller.yaw -= mouse_delta.x * controller.sensitivity;
            controller.pitch -= mouse_delta.y * controller.sensitivity;
            controller.pitch = controller.pitch.clamp(-1.54, 1.54);
            transform.rotation =
                Quat::from_euler(EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
        }

        let mut direction = Vec3::ZERO;
        let forward = transform.forward().as_vec3();
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let right = transform.right().as_vec3();
        let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

        if keyboard.pressed(KeyCode::KeyW) {
            direction += forward_flat;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction -= forward_flat;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction -= right_flat;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += right_flat;
        }
        if keyboard.pressed(KeyCode::Space) {
            direction += Vec3::Y;
        }
        if keyboard.pressed(KeyCode::ShiftLeft) {
            direction -= Vec3::Y;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * controller.speed * time.delta_secs();
        }
    }

    pub(crate) fn handle_scroll_adjust_distance(
        mut mouse_scroll: MessageReader<MouseWheel>,
        mut held_query: Query<&mut HeldMovable>,
    ) {
        let mut distance_delta = 0.0;
        for event in mouse_scroll.read() {
            distance_delta += event.y;
        }

        if distance_delta != 0.0 {
            for mut held in held_query.iter_mut() {
                let new_distance = held.distance + distance_delta * 0.5;
                held.distance = new_distance.clamp(0.5, 50.0);
            }
        }
    }
}
