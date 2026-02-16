use bevy::prelude::*;

pub mod amino_acids;
pub mod atoms;
pub mod rendering;

use rendering::RenderingPlugin;

pub struct ChemistryPlugin;

impl Plugin for ChemistryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderingPlugin);
    }
}
