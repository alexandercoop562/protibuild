use bevy::prelude::*;

use crate::{
    chemistry::ChemistryPlugin,
    objects::{dev_cube::DevCubePlugin, grid::GridPlugin},
    projects::{ProjectPlugin, ProjectResource, templates::ProjectTemplates},
    ui::UIPlugin,
    user::UserPlugin,
};

pub struct World;

impl World {
    pub fn init(app: &mut App) {
        app.add_plugins(GridPlugin);
        app.add_plugins(DevCubePlugin);
        app.add_plugins(ChemistryPlugin);
        app.add_plugins(ProjectPlugin);
        app.add_plugins(UserPlugin);
        app.add_plugins(UIPlugin);

        app.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 1.0)))
            .insert_resource(ProjectResource::new(ProjectTemplates::dev_cube()))
            .add_systems(Startup, Self::setup);
    }

    pub(crate) fn setup(mut commands: Commands) {
        commands.insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
            ..default()
        });
    }
}
