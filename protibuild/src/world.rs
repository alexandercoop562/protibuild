use bevy::prelude::*;

use crate::{
    objects::{dev_cube::DevCube, grid::Grid},
    ui::UI,
    user::User,
};

pub struct World;

impl World {
    pub fn init(app: &mut App) {
        Grid::init(app);
        DevCube::init(app);
        User::init(app);
        UI::init(app);

        app.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 1.0)))
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
