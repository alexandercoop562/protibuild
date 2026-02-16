use bevy::prelude::*;

use crate::chemistry::atoms::{Atom, Bond, Element};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rendering_resources)
            .add_systems(Update, render_atoms)
            .add_systems(PostUpdate, render_bonds);
    }
}

#[derive(Resource)]
pub struct RenderingMaterials {
    pub materials: Vec<Handle<StandardMaterial>>,
    pub sphere_mesh: Handle<Mesh>,
    pub cylinder_mesh: Handle<Mesh>,
}

fn setup_rendering_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let elements = [
        Element::Hydrogen,
        Element::Carbon,
        Element::Nitrogen,
        Element::Oxygen,
        Element::Sulfur,
    ];

    let element_materials: Vec<Handle<StandardMaterial>> = elements
        .iter()
        .map(|e| {
            materials.add(StandardMaterial {
                base_color: e.color(),
                metallic: 0.0,
                perceptual_roughness: 0.3,
                reflectance: 0.5,
                ..default()
            })
        })
        .collect();

    let sphere_mesh = meshes.add(
        Sphere::new(1.0)
            .mesh()
            .ico(16)
            .expect("Failed to create icosphere mesh"),
    );
    let cylinder_mesh = meshes.add(Cylinder::new(0.025, 1.0));

    commands.insert_resource(RenderingMaterials {
        materials: element_materials,
        sphere_mesh,
        cylinder_mesh,
    });
}

fn render_atoms(
    mut commands: Commands,
    query: Query<(Entity, &Atom, &Transform), Without<Mesh3d>>,
    rendering_materials: Res<RenderingMaterials>,
) {
    for (entity, atom, transform) in &query {
        let material_idx = match atom.element {
            Element::Hydrogen => 0,
            Element::Carbon => 1,
            Element::Nitrogen => 2,
            Element::Oxygen => 3,
            Element::Sulfur => 4,
        };

        let radius = atom.element.covalent_radius() * 0.3;

        commands.entity(entity).insert((
            Mesh3d(rendering_materials.sphere_mesh.clone()),
            MeshMaterial3d(rendering_materials.materials[material_idx].clone()),
            Transform::from_translation(transform.translation).with_scale(Vec3::splat(radius)),
        ));
    }
}

fn render_bonds(
    mut commands: Commands,
    bond_query: Query<(Entity, &Bond, &ChildOf), Without<Mesh3d>>,
    atom_query: Query<&GlobalTransform>,
    parent_query: Query<&GlobalTransform>,
    rendering_materials: Res<RenderingMaterials>,
) {
    for (entity, bond, child_of) in &bond_query {
        if let (Ok(atom_transform1), Ok(atom_transform2), Ok(parent_transform)) = (
            atom_query.get(bond.atom1),
            atom_query.get(bond.atom2),
            parent_query.get(child_of.parent()),
        ) {
            let pos1 = atom_transform1.translation();
            let pos2 = atom_transform2.translation();
            let mid_global = (pos1 + pos2) / 2.0;
            let direction = pos2 - pos1;
            let length = direction.length();

            if length > 0.0 {
                // Convert global position to local position relative to parent
                let parent_inv = parent_transform.to_matrix().inverse();
                let mid_local = parent_inv.transform_point3(mid_global);

                // Convert global direction to local direction
                let direction_local = parent_inv.transform_vector3(direction);
                let rotation = Quat::from_rotation_arc(Vec3::Y, direction_local.normalize());

                commands.entity(entity).insert((
                    Mesh3d(rendering_materials.cylinder_mesh.clone()),
                    MeshMaterial3d(rendering_materials.materials[1].clone()), // Carbon color for bonds
                    Transform::from_translation(mid_local)
                        .with_rotation(rotation)
                        .with_scale(Vec3::new(1.0, length, 1.0)),
                ));
            }
        }
    }
}
