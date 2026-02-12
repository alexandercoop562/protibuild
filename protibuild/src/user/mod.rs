use bevy::prelude::*;

pub(crate) mod camera;
pub(crate) mod interaction;

use camera::Camera;
use interaction::Interaction;

pub(crate) struct User;

impl User {
    pub fn init(app: &mut App) {
        Camera::init(app);
        Interaction::init(app);
    }
}
