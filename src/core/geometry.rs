
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GeometryAttributes {
    pub position: [f32; 3],
    pub color: Option<[f32; 3]>,
    pub tex_coords: Option<[f32; 3]>,
    pub normal: Option<[f32; 3]>,
}

struct Geometry {
    attributes: GeometryAttributes,
    indices: Option<Vec<u32>>,
}
