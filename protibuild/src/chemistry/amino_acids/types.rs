/// Standard amino acid codes (one-letter and three-letter representations).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AminoAcidCode {
    Gly,
    Ala,
    Ser,
    Cys,
    Pro,
    Val,
    Ile,
    Leu,
    Met,
    Phe,
    Tyr,
    Trp,
    Asn,
    Gln,
    Thr,
    Asp,
    Glu,
    Lys,
    Arg,
    His,
}

impl AminoAcidCode {
    pub fn name(&self) -> &'static str {
        match self {
            AminoAcidCode::Gly => "Glycine",
            AminoAcidCode::Ala => "Alanine",
            AminoAcidCode::Ser => "Serine",
            AminoAcidCode::Cys => "Cysteine",
            AminoAcidCode::Pro => "Proline",
            AminoAcidCode::Val => "Valine",
            AminoAcidCode::Ile => "Isoleucine",
            AminoAcidCode::Leu => "Leucine",
            AminoAcidCode::Met => "Methionine",
            AminoAcidCode::Phe => "Phenylalanine",
            AminoAcidCode::Tyr => "Tyrosine",
            AminoAcidCode::Trp => "Tryptophan",
            AminoAcidCode::Asn => "Asparagine",
            AminoAcidCode::Gln => "Glutamine",
            AminoAcidCode::Thr => "Threonine",
            AminoAcidCode::Asp => "Aspartic Acid",
            AminoAcidCode::Glu => "Glutamic Acid",
            AminoAcidCode::Lys => "Lysine",
            AminoAcidCode::Arg => "Arginine",
            AminoAcidCode::His => "Histidine",
        }
    }

    pub fn one_letter(&self) -> char {
        match self {
            AminoAcidCode::Gly => 'G',
            AminoAcidCode::Ala => 'A',
            AminoAcidCode::Ser => 'S',
            AminoAcidCode::Cys => 'C',
            AminoAcidCode::Pro => 'P',
            AminoAcidCode::Val => 'V',
            AminoAcidCode::Ile => 'I',
            AminoAcidCode::Leu => 'L',
            AminoAcidCode::Met => 'M',
            AminoAcidCode::Phe => 'F',
            AminoAcidCode::Tyr => 'Y',
            AminoAcidCode::Trp => 'W',
            AminoAcidCode::Asn => 'N',
            AminoAcidCode::Gln => 'Q',
            AminoAcidCode::Thr => 'T',
            AminoAcidCode::Asp => 'D',
            AminoAcidCode::Glu => 'E',
            AminoAcidCode::Lys => 'K',
            AminoAcidCode::Arg => 'R',
            AminoAcidCode::His => 'H',
        }
    }

    pub fn three_letter(&self) -> &'static str {
        match self {
            AminoAcidCode::Gly => "GLY",
            AminoAcidCode::Ala => "ALA",
            AminoAcidCode::Ser => "SER",
            AminoAcidCode::Cys => "CYS",
            AminoAcidCode::Pro => "PRO",
            AminoAcidCode::Val => "VAL",
            AminoAcidCode::Ile => "ILE",
            AminoAcidCode::Leu => "LEU",
            AminoAcidCode::Met => "MET",
            AminoAcidCode::Phe => "PHE",
            AminoAcidCode::Tyr => "TYR",
            AminoAcidCode::Trp => "TRP",
            AminoAcidCode::Asn => "ASN",
            AminoAcidCode::Gln => "GLN",
            AminoAcidCode::Thr => "THR",
            AminoAcidCode::Asp => "ASP",
            AminoAcidCode::Glu => "GLU",
            AminoAcidCode::Lys => "LYS",
            AminoAcidCode::Arg => "ARG",
            AminoAcidCode::His => "HIS",
        }
    }
}
