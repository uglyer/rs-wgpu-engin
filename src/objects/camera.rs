use cgmath::{Deg, Matrix4, perspective, SquareMatrix};

pub struct PerspectiveCamera {
	aspect: f32,
	far: f32,
	fovy: Deg<f32>,
	near: f32,
	projection_matrix: [[f32; 4]; 4],
	projection_matrix_inverse: [[f32; 4]; 4],
}

impl PerspectiveCamera {
	pub fn new(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
		let mut camera = PerspectiveCamera {
			aspect: aspect,
			far: far,
			fovy: Deg(fovy),
			near: near,
			projection_matrix: Matrix4::identity().into(),
			projection_matrix_inverse: Matrix4::identity().into(),
		};
		camera.update_projection_matrix();
		camera
	}

	pub fn set_aspect(&mut self, aspect: f32) -> &mut Self {
		self.aspect = aspect;
		self.update_projection_matrix();
		self
	}

	pub fn update_projection_matrix(&mut self) {
		let m = perspective(
			self.fovy,
			self.aspect,
			self.near,
			self.far,
		);
		self.projection_matrix.clone_from(&m.into());
		self.projection_matrix_inverse.clone_from(&m.invert().unwrap().into());
	}

	pub fn borrow_projection_matrix(&self) -> &[[f32; 4]; 4] {
		&self.projection_matrix
	}

	pub fn borrow_projection_matrix_inverse(&self) -> &[[f32; 4]; 4] {
		&self.projection_matrix_inverse
	}
}
