# Projects

Protibuild uses a project-based system that allows you to switch between different molecular scenes. Each project defines which objects are loaded and the initial camera position.

## Switching Projects

1. Click the **File** tab in the top-left corner of the screen
2. The File menu will open showing available templates
3. Click on a project button to load that scene
4. The view will automatically switch to the Default tab

## Available Templates

### Dev Cube

A simple development test project featuring:

- A single gray rotating cube at the world origin
- Useful for testing camera controls and object interaction
- Camera starts at position (0, 2, 5) looking at origin

This is the default project when the application starts.

### Amino Acids

Displays all 20 standard amino acids arranged in a 4-row grid layout:

#### Row 1: Small & Aliphatic
- Glycine (GLY)
- Alanine (ALA)
- Valine (VAL)
- Leucine (LEU)
- Isoleucine (ILE)
- Methionine (MET)
- Proline (PRO)

#### Row 2: Aromatic
- Phenylalanine (PHE)
- Tyrosine (TYR)
- Tryptophan (TRP)

#### Row 3: Polar & Charged
- Serine (SER)
- Threonine (THR)
- Cysteine (CYS)
- Asparagine (ASN)
- Glutamine (GLN)
- Aspartic Acid (ASP)
- Glutamic Acid (GLU)

#### Row 4: Basic
- Lysine (LYS)
- Arginine (ARG)
- Histidine (HIS)

Each amino acid is labeled with its three-letter code and positioned with appropriate spacing for easy viewing and comparison.

Camera starts at position (0, 10, 15) looking at the center of the grid.

## Project Structure

Projects are defined in code using the `Project` struct with:

- **Name**: Display name of the project
- **Description**: Brief description shown in the UI
- **Objects**: List of molecules to spawn
- **Camera**: Initial position and look target

### Adding New Projects

To add a new project template:

1. Define the objects using `ProjectObject` helpers
2. Set initial camera position and target
3. Add a button in the UI tab bar

See `projects/templates.rs` for implementation details.
