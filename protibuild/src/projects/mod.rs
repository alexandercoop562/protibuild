use bevy::prelude::*;

pub mod objects;
pub mod templates;

use objects::ProjectObject;

/// A project containing a collection of 3D objects to display.
#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub objects: Vec<ProjectObject>,
    pub camera_position: Vec3,
    pub camera_look_at: Vec3,
}

impl Project {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            objects: Vec::new(),
            camera_position: Vec3::new(-2.5, 4.5, 9.0),
            camera_look_at: Vec3::ZERO,
        }
    }

    pub fn with_object(mut self, object: ProjectObject) -> Self {
        self.objects.push(object);
        self
    }

    pub fn with_objects(mut self, objects: impl IntoIterator<Item = ProjectObject>) -> Self {
        self.objects.extend(objects);
        self
    }

    pub fn with_camera(mut self, position: Vec3, look_at: Vec3) -> Self {
        self.camera_position = position;
        self.camera_look_at = look_at;
        self
    }
}

/// Resource holding the currently active project.
#[derive(Resource, Clone, Debug)]
pub struct ProjectResource {
    pub project: Project,
}

impl ProjectResource {
    pub fn new(project: Project) -> Self {
        Self { project }
    }
}

/// Resource to track all project entity roots
#[derive(Resource, Default)]
pub struct ProjectEntities {
    pub roots: Vec<Entity>,
    pub initialized: bool,
}

/// Message to trigger project switching
#[derive(Message, Clone, Debug)]
pub struct SwitchProjectMessage {
    pub project: Project,
}

/// Plugin for managing projects and project switching.
pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectEntities>()
            .add_message::<SwitchProjectMessage>()
            .add_systems(Update, (handle_project_spawn, handle_project_switch));
    }
}

fn handle_project_spawn(
    mut commands: Commands,
    project_res: Res<ProjectResource>,
    mut project_entities: ResMut<ProjectEntities>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    if !project_entities.initialized {
        project_entities.initialized = true;

        if let Err(e) = spawn_project_objects(
            &mut commands,
            &project_res.project,
            &mut project_entities,
            &mut meshes,
            &mut materials,
        ) {
            error!("Failed to spawn project objects: {}", e);
            return;
        }

        if let Ok(mut camera_transform) = camera_query.single_mut() {
            *camera_transform = Transform::from_translation(project_res.project.camera_position)
                .looking_at(project_res.project.camera_look_at, Vec3::Y);
        }
    }
}

fn handle_project_switch(
    mut commands: Commands,
    mut project_res: ResMut<ProjectResource>,
    mut project_entities: ResMut<ProjectEntities>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut switch_messages: MessageReader<SwitchProjectMessage>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    for message in switch_messages.read() {
        for entity in project_entities.roots.drain(..) {
            commands.entity(entity).despawn();
        }

        *project_res = ProjectResource::new(message.project.clone());

        if let Err(e) = spawn_project_objects(
            &mut commands,
            &message.project,
            &mut project_entities,
            &mut meshes,
            &mut materials,
        ) {
            error!("Failed to spawn project objects: {}", e);
            return;
        }

        if let Ok(mut camera_transform) = camera_query.single_mut() {
            *camera_transform = Transform::from_translation(message.project.camera_position)
                .looking_at(message.project.camera_look_at, Vec3::Y);
        }
    }
}

fn spawn_project_objects(
    commands: &mut Commands,
    project: &Project,
    project_entities: &mut ProjectEntities,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Result<(), String> {
    for object in &project.objects {
        match object {
            ProjectObject::DevCube {
                position,
                rotation,
                scale,
            } => {
                let entity = crate::objects::dev_cube::spawn_dev_cube(
                    commands, meshes, materials, *position, *rotation, *scale,
                );
                project_entities.roots.push(entity);
            }
            ProjectObject::AminoAcid {
                code,
                position,
                residue_number,
            } => {
                let entity = crate::chemistry::amino_acids::AminoAcidBuilder::spawn(
                    commands,
                    *code,
                    *position,
                    *residue_number,
                )?;
                project_entities.roots.push(entity);
            }
        }
    }
    Ok(())
}
