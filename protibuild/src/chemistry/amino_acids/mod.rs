use bevy::prelude::*;

pub mod definitions;
pub mod types;

pub use types::AminoAcidCode;

use crate::chemistry::atoms::Bond;
use definitions::AminoAcidDefinition;

#[derive(Component)]
pub struct AminoAcid {
    pub code: AminoAcidCode,
}

#[derive(Component)]
pub struct Residue {
    pub residue_number: u32,
    pub chain_id: char,
}

#[derive(Bundle)]
pub struct AminoAcidBundle {
    pub amino_acid: AminoAcid,
    pub residue: Residue,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl AminoAcidBundle {
    pub fn new(code: AminoAcidCode, position: Vec3, residue_number: u32) -> Self {
        Self {
            amino_acid: AminoAcid { code },
            residue: Residue {
                residue_number,
                chain_id: 'A',
            },
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

pub struct AminoAcidBuilder;

impl AminoAcidBuilder {
    pub fn spawn(
        commands: &mut Commands,
        code: AminoAcidCode,
        position: Vec3,
        residue_number: u32,
    ) -> Result<Entity, String> {
        let definition = AminoAcidDefinition::get(code)?;

        let parent = commands
            .spawn((
                Name::new(format!("AminoAcid_{}", code.three_letter())),
                AminoAcid { code },
                Residue {
                    residue_number,
                    chain_id: 'A',
                },
                Transform::from_translation(position),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ))
            .id();

        let mut atom_entities: Vec<Entity> = Vec::new();
        for atom in &definition.atoms {
            let atom_entity = commands
                .spawn((
                    Name::new(format!("Atom_{}", atom.atom_name)),
                    atom.clone(),
                    Transform::from_translation(atom.position),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ))
                .id();

            commands.entity(parent).add_child(atom_entity);
            atom_entities.push(atom_entity);
        }

        for (idx1, idx2) in &definition.bonds {
            let bond_entity = commands
                .spawn((
                    Name::new(format!("Bond_{}_{}", idx1, idx2)),
                    Bond {
                        atom1: atom_entities[*idx1],
                        atom2: atom_entities[*idx2],
                    },
                    Transform::default(),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ))
                .id();

            commands.entity(parent).add_child(bond_entity);
        }

        Ok(parent)
    }
}
