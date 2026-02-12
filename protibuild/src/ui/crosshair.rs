use bevy::prelude::*;

use crate::user::camera::CameraController;

#[derive(Component)]
pub(crate) struct Crosshair;

impl Crosshair {
    pub(crate) fn init(app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::update);
    }

    pub(crate) fn setup(mut commands: Commands) {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    ..default()
                },
                Crosshair,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node {
                        width: Val::Px(2.0),
                        height: Val::Px(16.0),
                        position_type: PositionType::Absolute,
                        top: Val::Px(-8.0),
                        left: Val::Px(-1.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
                parent.spawn((
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(2.0),
                        position_type: PositionType::Absolute,
                        top: Val::Px(-1.0),
                        left: Val::Px(-8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
            });
    }

    pub(crate) fn update(
        controller_query: Query<&CameraController>,
        mut crosshair_query: Query<&mut Node, With<Crosshair>>,
    ) {
        let Ok(controller) = controller_query.single() else {
            return;
        };
        let Ok(mut style) = crosshair_query.single_mut() else {
            return;
        };

        style.display = if controller.is_captured {
            Display::Flex
        } else {
            Display::None
        };
    }
}
