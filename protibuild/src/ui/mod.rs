use bevy::prelude::*;

pub(crate) mod crosshair;

use crosshair::Crosshair;

pub(crate) struct UI;

impl UI {
    pub(crate) fn init(app: &mut App) {
        Crosshair::init(app);

        app.add_systems(Update, Self::handle_exit);
    }

    pub(crate) fn handle_exit(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut exit: MessageWriter<AppExit>,
    ) {
        if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyC) {
            exit.write(AppExit::Success);
        }
    }
}
