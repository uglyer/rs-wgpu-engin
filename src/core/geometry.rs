
// TODO 支持部分参数作为可选项
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GeometryAttributes {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 3],
    pub normal: [f32; 3],
}

struct Geometry {
    attributes: GeometryAttributes,
    indices: Option<Vec<u32>>,
}
