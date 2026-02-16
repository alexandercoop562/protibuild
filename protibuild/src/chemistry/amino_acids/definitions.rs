use bevy::prelude::*;

use super::types::AminoAcidCode;
use crate::chemistry::atoms::Atom;
use crate::config::AMINO_ACIDS_CONFIG;

/// Complete amino acid definition with atoms and bond connectivity.
pub struct AminoAcidDefinition {
    pub code: AminoAcidCode,
    pub atoms: Vec<Atom>,
    pub bonds: Vec<(usize, usize)>,
}

impl AminoAcidDefinition {
    pub fn get(code: AminoAcidCode) -> Result<Self, String> {
        let config_data = AMINO_ACIDS_CONFIG
            .as_ref()
            .map_err(|e| format!("Failed to load amino acids config: {}", e))?
            .get(&code)
            .ok_or_else(|| format!("Amino acid {:?} not found in config", code))?;

        let atoms: Vec<Atom> = config_data
            .atoms
            .iter()
            .map(|atom_config| {
                atom_config.to_element().map(|element| {
                    Atom::new(
                        element,
                        Vec3::new(atom_config.x, atom_config.y, atom_config.z),
                        atom_config.name.clone(),
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let bonds = config_data.bonds.clone();

        Ok(Self { code, atoms, bonds })
    }
}
