use bevy::prelude::*;

/// Chemical elements supported by the simulator.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Element {
    Hydrogen,
    Carbon,
    Nitrogen,
    Oxygen,
    Sulfur,
}

impl Element {
    pub fn covalent_radius(&self) -> f32 {
        match self {
            Element::Hydrogen => 0.31,
            Element::Carbon => 0.76,
            Element::Nitrogen => 0.71,
            Element::Oxygen => 0.66,
            Element::Sulfur => 1.05,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Element::Hydrogen => Color::srgb(1.0, 1.0, 1.0),
            Element::Carbon => Color::srgb(0.4, 0.4, 0.4),
            Element::Nitrogen => Color::srgb(0.0, 0.0, 0.8),
            Element::Oxygen => Color::srgb(0.8, 0.0, 0.0),
            Element::Sulfur => Color::srgb(0.9, 0.8, 0.0),
        }
    }
}

/// A single atom with element type, position, and name.
#[derive(Component, Clone, Debug)]
pub struct Atom {
    pub element: Element,
    pub position: Vec3,
    pub atom_name: String,
}

impl Atom {
    pub fn new(element: Element, position: Vec3, atom_name: impl Into<String>) -> Self {
        Self {
            element,
            position,
            atom_name: atom_name.into(),
        }
    }
}

#[derive(Bundle)]
pub struct AtomBundle {
    pub atom: Atom,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl AtomBundle {
    pub fn new(atom: Atom) -> Self {
        let position = atom.position;
        Self {
            atom,
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

/// A bond connecting two atoms.
#[derive(Component)]
pub struct Bond {
    pub atom1: Entity,
    pub atom2: Entity,
}

#[derive(Bundle)]
pub struct BondBundle {
    pub bond: Bond,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl BondBundle {
    pub fn new(bond: Bond) -> Self {
        Self {
            bond,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
