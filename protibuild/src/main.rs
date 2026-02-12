use bevy::prelude::*;

use protibuild::world::World;

fn main() {
    let mut app = App::new();
    World::init(&mut app);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }))
    .run();
}
