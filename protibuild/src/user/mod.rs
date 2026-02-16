use bevy::prelude::*;

pub(crate) mod camera;
pub(crate) mod interaction;

use camera::CameraPlugin;
use interaction::Interaction;

pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
        Interaction::init(app);
    }
}
