use crate::chemistry::amino_acids::types::AminoAcidCode;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum ProjectObject {
    DevCube {
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    },
    AminoAcid {
        code: AminoAcidCode,
        position: Vec3,
        residue_number: u32,
    },
}

impl ProjectObject {
    pub fn dev_cube() -> Self {
        Self::DevCube {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn dev_cube_at(position: Vec3) -> Self {
        Self::DevCube {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn dev_cube_with_transform(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self::DevCube {
            position,
            rotation,
            scale,
        }
    }

    pub fn amino_acid(code: AminoAcidCode, position: Vec3, residue_number: u32) -> Self {
        Self::AminoAcid {
            code,
            position,
            residue_number,
        }
    }
}
