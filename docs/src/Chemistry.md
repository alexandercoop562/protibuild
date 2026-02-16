# Chemistry

Protibuild provides accurate molecular visualization using the ball-and-stick model. This section documents the chemical elements and amino acids supported by the application.

## Elements

Protibuild supports 5 common elements found in organic molecules and proteins:

| Element | Symbol | Color | Covalent Radius (Å) |
|---------|--------|-------|---------------------|
| Hydrogen | H | White | 0.31 |
| Carbon | C | Gray | 0.76 |
| Nitrogen | N | Blue | 0.71 |
| Oxygen | O | Red | 0.66 |
| Sulfur | S | Yellow | 1.05 |

### CPK Coloring

The application uses standard CPK (Corey-Pauling-Koltun) coloring, which is the industry standard for molecular visualization:

- **Hydrogen (H)**: White - Represented as small white spheres
- **Carbon (C)**: Gray - The backbone of organic molecules
- **Nitrogen (N)**: Blue - Found in amino groups
- **Oxygen (O)**: Red - Found in carbonyl and hydroxyl groups
- **Sulfur (S)**: Yellow - Found in cysteine and methionine

### Visualization

- **Atoms** are rendered as spheres with radii proportional to their covalent radii (scaled by 0.3 for visibility)
- **Bonds** are rendered as gray cylinders connecting atoms
- Atom size reflects the element type - Hydrogen is smallest, Sulfur is largest

## Amino Acids

Protibuild supports all 20 standard amino acids found in proteins. Each amino acid is modeled with accurate atomic positions and bond connectivity.

### Amino Acid Reference

| Code | One-Letter | Full Name | Category |
|------|------------|-----------|----------|
| GLY | G | Glycine | Small |
| ALA | A | Alanine | Small |
| SER | S | Serine | Polar |
| CYS | C | Cysteine | Polar |
| PRO | P | Proline | Small |
| VAL | V | Valine | Aliphatic |
| ILE | I | Isoleucine | Aliphatic |
| LEU | L | Leucine | Aliphatic |
| MET | M | Methionine | Aliphatic |
| PHE | F | Phenylalanine | Aromatic |
| TYR | Y | Tyrosine | Aromatic |
| TRP | W | Tryptophan | Aromatic |
| ASN | N | Asparagine | Polar |
| GLN | Q | Glutamine | Polar |
| THR | T | Threonine | Polar |
| ASP | D | Aspartic Acid | Acidic |
| GLU | E | Glutamic Acid | Acidic |
| LYS | K | Lysine | Basic |
| ARG | R | Arginine | Basic |
| HIS | H | Histidine | Basic |

### Structure

Each amino acid is rendered as a parent entity containing:

1. **Atom entities** - Individual atoms positioned at their correct 3D coordinates
2. **Bond entities** - Cylinders connecting bonded atoms
3. **Metadata** - Amino acid code, residue number, and chain ID

### Data Source

Amino acid structures are loaded from a JSON configuration file (`amino_acids.json`) which contains:

- Atom positions (x, y, z coordinates)
- Atom names (e.g., "CA", "CB", "N", "O")
- Bond connectivity (pairs of atom indices)

This allows easy extension with additional molecules or modified structures.
