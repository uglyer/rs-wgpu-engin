use crate::core::geometry::Geometry;
use crate::core::resource::ResourceId;
use crate::materials::material::Material;

pub struct Mesh {
    geometry: ResourceId<Geometry>,
    material: ResourceId<Material>,
}

impl Mesh {
    pub fn new(
        geometry: ResourceId<Geometry>,
        material: ResourceId<Material>,
    ) -> Self {
        Mesh {
            geometry,
            material,
        }
    }

    pub fn borrow_geometry(&self) -> &ResourceId<Geometry> {
        &self.geometry
    }

    pub fn borrow_material(&self) -> &ResourceId<Material> {
        &self.material
    }
}
