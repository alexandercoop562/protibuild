use bevy::prelude::*;

pub(crate) mod crosshair;
pub(crate) mod tab_bar;

use crosshair::CrosshairPlugin;
use tab_bar::TabBarPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CrosshairPlugin)
            .add_plugins(TabBarPlugin)
            .add_systems(Update, Self::handle_exit);
    }
}

impl UIPlugin {
    fn handle_exit(keyboard: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
        if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyC) {
            exit.write(AppExit::Success);
        }
    }
}
