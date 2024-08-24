use cgmath::Vector3;
use crate::core::attribute::{AttributeF32, AttributeUsize};
use crate::core::geometry::Geometry;
use crate::core::resource::{ResourceId, ResourcePools};

pub struct GeometryHelper {}

impl GeometryHelper {
    pub fn create_triangle(
        pools: &mut ResourcePools,
        width: f32,
        height: f32,
    ) -> ResourceId<Geometry> {
        let dy = 0.75_f32.sqrt() / 2.0;
        let positions = [
            0.0, (0.75_f32.sqrt() - dy) * height, 0.0,
            -0.5 * width, -dy * height, 0.0,
            0.5 * width, -dy * height, 0.0,
        ].to_vec();

        let normals = [
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
        ].to_vec();

        let uvs = [
            0.5, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ].to_vec();

        let indices = [
            0, 1, 2,
        ].to_vec();

        let mut geometry = Geometry::new();
        geometry.set_attribute("position", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(positions, 3)));
        geometry.set_attribute("normal", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(normals, 3)));
        geometry.set_attribute("uv", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(uvs, 2)));
        geometry.set_index(pools.borrow_mut::<AttributeUsize>().add(AttributeUsize::new(indices, 3)));
        pools.borrow_mut::<Geometry>().add(geometry)
    }

    pub fn create_plane(
        pools: &mut ResourcePools,
        width: f32,
        height: f32,
    ) -> ResourceId<Geometry> {
        let positions = [
            // top-left
            -0.5 * width, 0.5 * height, 0.0,
            // top-right
            0.5 * width, 0.5 * height, 0.0,
            // bottom-left
            -0.5 * width, -0.5 * height, 0.0,
            // bottom-right
            0.5 * width, -0.5 * height, 0.0,
        ].to_vec();

        let normals = [
            // top-left
            0.0, 0.0, 1.0,
            // top-right
            0.0, 0.0, 1.0,
            // bottom-left
            0.0, 0.0, 1.0,
            // bottom-right
            0.0, 0.0, 1.0,
        ].to_vec();

        let uvs = [
            // top-left
            0.0, 0.0,
            // top-right
            1.0, 0.0,
            // bottom-left
            0.0, 1.0,
            // bottom-right
            1.0, 1.0,
        ].to_vec();

        let indices = [
            0, 2, 1,
            1, 2, 3,
        ].to_vec();

        let mut geometry = Geometry::new();
        geometry.set_attribute("position", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(positions, 3)));
        geometry.set_attribute("normal", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(normals, 3)));
        geometry.set_attribute("uv", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(uvs, 2)));
        geometry.set_index(pools.borrow_mut::<AttributeUsize>().add(AttributeUsize::new(indices, 3)));
        pools.borrow_mut::<Geometry>().add(geometry)
    }

    pub fn create_box(
        pools: &mut ResourcePools,
        width: f32,
        height: f32,
        depth: f32,
    ) -> ResourceId<Geometry> {
        let mut positions: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut uvs: Vec<f32> = Vec::new();
        let mut indices: Vec<usize> = Vec::new();

        let mut position_vec = Vector3::new(0.0_f32, 0.0, 0.0);

        // @TODO: Clean up
        for face in 0..6 {
            let (x, y, z, nx, ny, nz, dx, dy, dz) = match face {
                // front
                0 => (
                    -0.5, 0.5, 0.5,
                    0.0, 0.0, 1.0,
                    [0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.0, -1.0, -1.0],
                    [0.0, 0.0, 0.0, 0.0],
                ),
                // right
                1 => (
                    0.5, 0.5, 0.5,
                    1.0, 0.0, 0.0,
                    [0.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, -1.0, -1.0],
                    [0.0, -1.0, 0.0, -1.0],
                ),
                // back
                2 => (
                    0.5, 0.5, -0.5,
                    0.0, 0.0, -1.0,
                    [0.0, -1.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0, -1.0],
                    [0.0, 0.0, 0.0, 0.0],
                ),
                // left
                3 => (
                    -0.5, 0.5, -0.5,
                    -1.0, 0.0, 0.0,
                    [0.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, -1.0, -1.0],
                    [0.0, 1.0, 0.0, 1.0],
                ),
                // top
                4 => (
                    -0.5, 0.5, -0.5,
                    0.0, 1.0, 0.0,
                    [0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 1.0],
                ),
                // bottom
                _ => (
                    -0.5, -0.5, 0.5,
                    0.0, -1.0, 0.0,
                    [0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, -1.0, -1.0],
                ),
            };

            for i in 0..4 {
                position_vec[0] = (x + dx[i]) * width;
                position_vec[1] = (y + dy[i]) * height;
                position_vec[2] = (z + dz[i]) * depth;

                for j in 0..3 {
                    positions.push(position_vec[j]);
                }

                normals.push(nx);
                normals.push(ny);
                normals.push(nz);
            }

            uvs.push(0.0);
            uvs.push(0.0);

            uvs.push(1.0);
            uvs.push(0.0);

            uvs.push(0.0);
            uvs.push(1.0);

            uvs.push(1.0);
            uvs.push(1.0);

            indices.push(face * 4 + 0);
            indices.push(face * 4 + 2);
            indices.push(face * 4 + 1);

            indices.push(face * 4 + 1);
            indices.push(face * 4 + 2);
            indices.push(face * 4 + 3);
        }

        let mut geometry = Geometry::new();
        geometry.set_attribute("position", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(positions, 3)));
        geometry.set_attribute("normal", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(normals, 3)));
        geometry.set_attribute("uv", pools.borrow_mut::<AttributeF32>().add(AttributeF32::new(uvs, 2)));
        geometry.set_index(pools.borrow_mut::<AttributeUsize>().add(AttributeUsize::new(indices, 3)));
        pools.borrow_mut::<Geometry>().add(geometry)
    }
}
