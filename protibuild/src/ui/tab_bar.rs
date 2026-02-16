use bevy::prelude::*;

use crate::projects::{SwitchProjectMessage, templates::ProjectTemplates};

const ACTIVE_TAB_COLOR: Color = Color::srgb(0.30, 0.30, 0.32);
const INACTIVE_TAB_COLOR: Color = Color::srgb(0.18, 0.18, 0.20);
const BUTTON_COLOR: Color = Color::srgb(0.25, 0.35, 0.45);
const BUTTON_HOVER_COLOR: Color = Color::srgb(0.35, 0.45, 0.55);
const TEXT_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);
const SECONDARY_TEXT_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// Resource tracking the currently active tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource)]
pub struct ActiveTab {
    pub tab: TabType,
}

/// Types of tabs available in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabType {
    #[default]
    Default,
    File,
}

#[derive(Component)]
struct TabBarRoot;

#[derive(Component)]
struct TabButton {
    tab: TabType,
}

#[derive(Component)]
struct FileTabContent;

#[derive(Component)]
struct ProjectSelectionButton {
    template: ProjectTemplate,
}

#[derive(Debug, Clone, Copy)]
enum ProjectTemplate {
    DevCube,
    AminoAcids,
}

/// Plugin for the tab bar UI.
pub struct TabBarPlugin;

impl Plugin for TabBarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveTab { tab: TabType::File })
            .add_systems(Startup, setup_tab_bar)
            .add_systems(
                Update,
                (
                    handle_tab_clicks,
                    handle_project_selection,
                    update_tab_visibility,
                ),
            );
    }
}

fn setup_tab_bar(mut commands: Commands) {
    commands
        .spawn((
            Name::new("TabBar"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(32.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            BackgroundColor(INACTIVE_TAB_COLOR),
            TabBarRoot,
            Visibility::Visible,
        ))
        .with_children(|parent| {
            // File tab button - active by default, styled like a file tab
            parent
                .spawn((
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(28.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Px(8.0),
                            right: Val::Px(2.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                        },
                        border_radius: BorderRadius {
                            top_left: Val::Px(6.0),
                            top_right: Val::Px(6.0),
                            bottom_left: Val::Px(0.0),
                            bottom_right: Val::Px(0.0),
                        },
                        ..default()
                    },
                    BackgroundColor(ACTIVE_TAB_COLOR),
                    TabButton { tab: TabType::File },
                    Interaction::None,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("File"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });

            // Default tab button
            parent
                .spawn((
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(24.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Px(0.0),
                            right: Val::Px(2.0),
                            top: Val::Px(4.0),
                            bottom: Val::Px(0.0),
                        },
                        border_radius: BorderRadius {
                            top_left: Val::Px(4.0),
                            top_right: Val::Px(4.0),
                            bottom_left: Val::Px(0.0),
                            bottom_right: Val::Px(0.0),
                        },
                        ..default()
                    },
                    BackgroundColor(INACTIVE_TAB_COLOR),
                    TabButton {
                        tab: TabType::Default,
                    },
                    Interaction::None,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Default"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
        });

    commands
        .spawn((
            Name::new("FileTabContent"),
            Node {
                width: Val::Px(300.0),
                height: Val::Vh(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(32.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BorderColor::all(Color::srgba(0.25, 0.25, 0.28, 1.0)),
            BackgroundColor(Color::srgba(0.12, 0.12, 0.14, 0.98)),
            FileTabContent,
            Visibility::Visible,
        ))
        .with_children(|parent| {
            // Title - App name
            parent.spawn((
                Text::new("Protibuild"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Templates label
            parent.spawn((
                Text::new("Templates:"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(SECONDARY_TEXT_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Dev Cube button
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(BUTTON_COLOR),
                    ProjectSelectionButton {
                        template: ProjectTemplate::DevCube,
                    },
                    Interaction::None,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Dev Cube"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(BUTTON_TEXT_COLOR),
                    ));
                });

            // Amino Acids button
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(BUTTON_COLOR),
                    ProjectSelectionButton {
                        template: ProjectTemplate::AminoAcids,
                    },
                    Interaction::None,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Amino Acids"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(BUTTON_TEXT_COLOR),
                    ));
                });
        });
}

fn handle_tab_clicks(
    mut active_tab: ResMut<ActiveTab>,
    interaction_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    mut tab_button_query: Query<(&TabButton, &mut BackgroundColor, &mut Node)>,
) {
    for (interaction, button) in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                active_tab.tab = button.tab;

                for (btn, mut bg_color, mut node) in &mut tab_button_query {
                    if btn.tab == active_tab.tab {
                        *bg_color = BackgroundColor(ACTIVE_TAB_COLOR);
                        node.height = Val::Px(28.0);
                        node.margin.top = Val::Px(0.0);
                    } else {
                        *bg_color = BackgroundColor(INACTIVE_TAB_COLOR);
                        node.height = Val::Px(24.0);
                        node.margin.top = Val::Px(4.0);
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn update_tab_visibility(
    active_tab: Res<ActiveTab>,
    mut file_content_query: Query<&mut Visibility, With<FileTabContent>>,
) {
    if active_tab.is_changed()
        && let Ok(mut visibility) = file_content_query.single_mut()
    {
        *visibility = if active_tab.tab == TabType::File {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[allow(clippy::type_complexity)]
fn handle_project_selection(
    mut messages: MessageWriter<SwitchProjectMessage>,
    mut active_tab: ResMut<ActiveTab>,
    interaction_query: Query<(&Interaction, &ProjectSelectionButton), Changed<Interaction>>,
    mut param_set: ParamSet<(
        Query<(&TabButton, &mut BackgroundColor, &mut Node)>,
        Query<(&ProjectSelectionButton, &mut BackgroundColor)>,
    )>,
) {
    for (interaction, button) in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let new_project = match button.template {
                    ProjectTemplate::DevCube => ProjectTemplates::dev_cube(),
                    ProjectTemplate::AminoAcids => ProjectTemplates::amino_acids(),
                };

                messages.write(SwitchProjectMessage {
                    project: new_project,
                });

                // Switch back to default tab
                active_tab.tab = TabType::Default;

                // Update tab button colors and height
                for (btn, mut bg_color, mut node) in &mut param_set.p0() {
                    if btn.tab == active_tab.tab {
                        *bg_color = BackgroundColor(ACTIVE_TAB_COLOR);
                        node.height = Val::Px(28.0);
                        node.margin.top = Val::Px(0.0);
                    } else {
                        *bg_color = BackgroundColor(INACTIVE_TAB_COLOR);
                        node.height = Val::Px(24.0);
                        node.margin.top = Val::Px(4.0);
                    }
                }
            }
            Interaction::Hovered => {
                // Highlight on hover
                for (btn, mut bg_color) in &mut param_set.p1() {
                    if std::mem::discriminant(&btn.template)
                        == std::mem::discriminant(&button.template)
                    {
                        *bg_color = BackgroundColor(BUTTON_HOVER_COLOR);
                    }
                }
            }
            Interaction::None => {
                // Reset color when not hovering
                for (btn, mut bg_color) in &mut param_set.p1() {
                    if std::mem::discriminant(&btn.template)
                        == std::mem::discriminant(&button.template)
                    {
                        *bg_color = BackgroundColor(BUTTON_COLOR);
                    }
                }
            }
        }
    }
}
