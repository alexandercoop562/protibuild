use bevy::prelude::*;

use super::{Project, objects::ProjectObject};
use crate::chemistry::amino_acids::types::AminoAcidCode;

pub struct ProjectTemplates;

impl ProjectTemplates {
    pub fn dev_cube() -> Project {
        Project::new(
            "Dev Cube",
            "Simple project with a single rotating dev cube at the origin",
        )
        .with_object(ProjectObject::dev_cube())
        .with_camera(Vec3::new(0.0, 2.0, 5.0), Vec3::ZERO)
    }

    pub fn amino_acids() -> Project {
        let mut project = Project::new(
            "Amino Acids",
            "All 20 standard amino acids arranged in a 4-row grid",
        )
        .with_camera(Vec3::new(0.0, 10.0, 15.0), Vec3::ZERO);

        let x_spacing = 5.0;
        let z_spacing = 6.0;
        let start_x = -18.0;
        let start_z = -9.0;

        // Row 1: Small and aliphatic amino acids
        project = project.with_objects([
            ProjectObject::amino_acid(AminoAcidCode::Gly, Vec3::new(start_x, 1.0, start_z), 1),
            ProjectObject::amino_acid(
                AminoAcidCode::Ala,
                Vec3::new(start_x + x_spacing, 1.0, start_z),
                2,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Val,
                Vec3::new(start_x + 2.0 * x_spacing, 1.0, start_z),
                3,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Leu,
                Vec3::new(start_x + 3.0 * x_spacing, 1.0, start_z),
                4,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Ile,
                Vec3::new(start_x + 4.0 * x_spacing, 1.0, start_z),
                5,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Met,
                Vec3::new(start_x + 5.0 * x_spacing, 1.0, start_z),
                6,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Pro,
                Vec3::new(start_x + 6.0 * x_spacing, 1.0, start_z),
                7,
            ),
        ]);

        // Row 2: Aromatic amino acids
        project = project.with_objects([
            ProjectObject::amino_acid(
                AminoAcidCode::Phe,
                Vec3::new(start_x, 1.0, start_z + z_spacing),
                8,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Tyr,
                Vec3::new(start_x + 2.0 * x_spacing, 1.0, start_z + z_spacing),
                9,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Trp,
                Vec3::new(start_x + 4.0 * x_spacing, 1.0, start_z + z_spacing),
                10,
            ),
        ]);

        // Row 3: Polar and charged amino acids
        project = project.with_objects([
            ProjectObject::amino_acid(
                AminoAcidCode::Ser,
                Vec3::new(start_x, 1.0, start_z + 2.0 * z_spacing),
                11,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Thr,
                Vec3::new(start_x + x_spacing, 1.0, start_z + 2.0 * z_spacing),
                12,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Cys,
                Vec3::new(start_x + 2.0 * x_spacing, 1.0, start_z + 2.0 * z_spacing),
                13,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Asn,
                Vec3::new(start_x + 3.0 * x_spacing, 1.0, start_z + 2.0 * z_spacing),
                14,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Gln,
                Vec3::new(start_x + 4.0 * x_spacing, 1.0, start_z + 2.0 * z_spacing),
                15,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Asp,
                Vec3::new(start_x + 5.0 * x_spacing, 1.0, start_z + 2.0 * z_spacing),
                16,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Glu,
                Vec3::new(start_x + 6.0 * x_spacing, 1.0, start_z + 2.0 * z_spacing),
                17,
            ),
        ]);

        // Row 4: Basic amino acids
        project.with_objects([
            ProjectObject::amino_acid(
                AminoAcidCode::Lys,
                Vec3::new(start_x, 1.0, start_z + 3.0 * z_spacing),
                18,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::Arg,
                Vec3::new(start_x + 2.0 * x_spacing, 1.0, start_z + 3.0 * z_spacing),
                19,
            ),
            ProjectObject::amino_acid(
                AminoAcidCode::His,
                Vec3::new(start_x + 4.0 * x_spacing, 1.0, start_z + 3.0 * z_spacing),
                20,
            ),
        ])
    }
}
