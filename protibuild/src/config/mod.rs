use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::chemistry::amino_acids::types::AminoAcidCode;
use crate::chemistry::atoms::Element;

/// Configuration for a single atom parsed from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomConfig {
    pub element: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub name: String,
}

/// Configuration for a single amino acid parsed from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AminoAcidConfig {
    pub code: String,
    pub atoms: Vec<AtomConfig>,
    pub bonds: Vec<(usize, usize)>,
}

/// Top-level configuration containing all amino acid definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AminoAcidsConfig {
    pub amino_acids: HashMap<String, AminoAcidConfig>,
}

impl AminoAcidsConfig {
    pub fn load() -> Result<Self, serde_json::Error> {
        const JSON_DATA: &str = include_str!("amino_acids.json");
        serde_json::from_str(JSON_DATA)
    }

    pub fn get(&self, code: &AminoAcidCode) -> Option<&AminoAcidConfig> {
        let code_str = match code {
            AminoAcidCode::Gly => "Gly",
            AminoAcidCode::Ala => "Ala",
            AminoAcidCode::Ser => "Ser",
            AminoAcidCode::Cys => "Cys",
            AminoAcidCode::Pro => "Pro",
            AminoAcidCode::Val => "Val",
            AminoAcidCode::Ile => "Ile",
            AminoAcidCode::Leu => "Leu",
            AminoAcidCode::Met => "Met",
            AminoAcidCode::Phe => "Phe",
            AminoAcidCode::Tyr => "Tyr",
            AminoAcidCode::Trp => "Trp",
            AminoAcidCode::Asn => "Asn",
            AminoAcidCode::Gln => "Gln",
            AminoAcidCode::Thr => "Thr",
            AminoAcidCode::Asp => "Asp",
            AminoAcidCode::Glu => "Glu",
            AminoAcidCode::Lys => "Lys",
            AminoAcidCode::Arg => "Arg",
            AminoAcidCode::His => "His",
        };
        self.amino_acids.get(code_str)
    }
}

impl AtomConfig {
    pub fn to_element(&self) -> Result<Element, String> {
        match self.element.as_str() {
            "H" => Ok(Element::Hydrogen),
            "C" => Ok(Element::Carbon),
            "N" => Ok(Element::Nitrogen),
            "O" => Ok(Element::Oxygen),
            "S" => Ok(Element::Sulfur),
            _ => Err(format!("Unknown element: {}", self.element)),
        }
    }
}

pub static AMINO_ACIDS_CONFIG: Lazy<Result<AminoAcidsConfig, serde_json::Error>> =
    Lazy::new(AminoAcidsConfig::load);
