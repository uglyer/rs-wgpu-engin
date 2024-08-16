use crate::core::resource::{ResourceId, ResourcePool};
use cgmath::{Matrix4, Vector3, Quaternion, Euler, SquareMatrix, Decomposed};
use crate::math::cgmath::{compose_matrix_to, decompose_matrix, decompose_matrix_to};

pub struct Node {
    children: Vec<ResourceId<Node>>,
    matrix: [[f32; 4]; 4],
    parent: Option<ResourceId<Node>>,
    position: [f32; 3],
    quaternion: [f32; 4],
    scale: [f32; 3],
    world_matrix: [[f32; 4]; 4],
}

impl Node {
    pub fn new() -> Self {
        Node {
            children: Vec::new(),
            matrix: Matrix4::identity().into(),
            parent: None,
            position: Vector3::new(0.0_f32, 0.0, 0.0).into(),
            quaternion: Quaternion::new(0.0, 0.0, 0.0, 1.0).into(),
            scale: Vector3::new(1.0_f32, 1.0, 1.0).into(),
            world_matrix: Matrix4::identity().into(),
        }
    }

    pub fn borrow_parent(&self) -> Option<&ResourceId<Node>> {
        self.parent.as_ref()
    }

    pub fn borrow_children(&self) -> &Vec<ResourceId<Node>> {
        &self.children
    }

    pub fn borrow_position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn borrow_position_mut(&mut self) -> &mut [f32; 3] {
        &mut self.position
    }

    // TODO 通过四元数转换
    // pub fn get_rotation(&self) -> &[f32; 3] {
    //     &self.quaternion
    // }
    //
    // pub fn borrow_rotation_mut(&mut self) -> &mut [f32; 3] {
    //     &mut self.rotation
    // }

    pub fn borrow_scale(&self) -> &[f32; 3] {
        &self.scale
    }

    pub fn borrow_scale_mut(&mut self) -> &mut [f32; 3] {
        &mut self.scale
    }

    pub fn borrow_matrix(&self) -> &[[f32; 4]; 4] {
        &self.matrix
    }

    pub fn borrow_world_matrix(&self) -> &[[f32; 4]; 4] {
        &self.world_matrix
    }

    pub fn set_matrix(&mut self, matrix: &[[f32; 4]; 4]) -> &mut Self {
        self.matrix.clone_from(matrix);
        decompose_matrix_to(matrix, &mut self.position, &mut self.quaternion, &mut self.scale);
        self
    }

    pub fn set_world_matrix(&mut self, matrix: &[[f32; 4]; 4]) -> &mut Self {
        self.world_matrix.clone_from(matrix);
        self
    }

    pub fn update_matrix(&mut self) -> &mut Self {
        compose_matrix_to(&mut self.matrix, &self.position, &self.quaternion, &self.scale);
        self
    }

    // @TODO: Optimize
    pub fn update_matrices(
        &mut self,
        pool: &mut ResourcePool<Node>,
    ) {
        self.update_matrix();

        if let Some(parent) = self.borrow_parent() {
            let parent_matrix = pool.borrow(parent).unwrap().borrow_world_matrix();
            let matrix = Matrix4::from(*parent_matrix) * Matrix4::from(self.matrix);
            self.world_matrix.clone_from(&matrix.into());
        } else {
            self.world_matrix.clone_from(&self.matrix);
        }

        let mut stack = Vec::new();

        for child in self.children.iter() {
            stack.push(*child);
        }

        while let Some(rid) = stack.pop() {
            let parent_matrix = {
                let node = pool.borrow_mut(&rid).unwrap();
                let parent = node.borrow_parent().cloned().unwrap();
                Matrix4::from(*pool.borrow(&parent).unwrap().borrow_world_matrix())
            };


            let node = pool.borrow_mut(&rid).unwrap();
            let matrix: Matrix4<f32> = Matrix4::from(parent_matrix) * Matrix4::from(*node.borrow_matrix());
            node.set_world_matrix(&matrix.into());

            for child in node.children.iter() {
                stack.push(*child);
            }
        }
    }
}

pub struct NodeExecutor {}

impl NodeExecutor {
    pub fn update_matrices(
        pool: &mut ResourcePool<Node>,
        root: &ResourceId<Node>,
    ) {
        let mut stack = Vec::new();
        stack.push(*root);

        while let Some(rid) = stack.pop() {
            let node = pool.borrow_mut(&rid).unwrap();
            node.update_matrix();

            let parent_matrix = {
                let node = pool.borrow_mut(&rid).unwrap();
                if let Some(parent) = node.borrow_parent().cloned() {
                    let m = pool.borrow(&parent).unwrap().borrow_world_matrix();
                    Matrix4::from(*m)
                } else{
                    Matrix4::identity()
                }
            };

            let node = pool.borrow_mut(&rid).unwrap();
            let matrix: Matrix4<f32> = Matrix4::from(parent_matrix) * Matrix4::from(*node.borrow_matrix());
            node.set_world_matrix(&matrix.into());

            for child in node.children.iter() {
                stack.push(*child);
            }
        }
    }

    pub fn collect_nodes(
        pool: &ResourcePool<Node>,
        root: &ResourceId<Node>,
        nodes: &mut Vec<ResourceId<Node>>,
    ) {
        let mut stack = Vec::new();
        stack.push(*root);
        nodes.push(*root);

        while let Some(rid) = stack.pop() {
            let node = pool.borrow(&rid).unwrap();
            for child in node.children.iter() {
                stack.push(*child);
                nodes.push(*child);
            }
        }
    }
}
