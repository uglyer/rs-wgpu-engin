use cgmath::{InnerSpace, Matrix3, Matrix4, Quaternion, Vector3};

// 提取矩阵的平移、旋转和缩放
pub fn decompose_matrix(m: &[[f32; 4]; 4]) -> (Vector3<f32>, Quaternion<f32>, Vector3<f32>) {
    let matrix: Matrix4<f32> = Matrix4::from(*m);
    // 提取缩放
    let scale_x = matrix.x.truncate().magnitude();
    let scale_y = matrix.y.truncate().magnitude();
    let scale_z = matrix.z.truncate().magnitude();
    let scale = Vector3::new(scale_x, scale_y, scale_z);

    // 归一化缩放后的矩阵以提取旋转
    let rotation_matrix = Matrix3::from_cols(
        matrix.x.truncate() / scale_x,
        matrix.y.truncate() / scale_y,
        matrix.z.truncate() / scale_z,
    );

    let rotation = Quaternion::from(rotation_matrix);

    // 提取位移
    let position = Vector3::new(matrix.w.x, matrix.w.y, matrix.w.z);

    (position, rotation, scale)
}

// 提取矩阵的平移、旋转和缩放
pub fn decompose_matrix_to(m: &[[f32; 4]; 4], position: &mut [f32; 3], quaternion: &mut [f32; 4], scale: &mut [f32; 3]) {
    let (p, q, s) = decompose_matrix(m);
    position.clone_from(&p.into());
    quaternion.clone_from(&q.into());
    scale.clone_from(&s.into());
}

pub fn compose_matrix(position: &[f32; 3], quaternion: &[f32; 4], scale: &[f32; 3]) -> [[f32; 4]; 4] {
    (Matrix4::from_translation(Vector3::new(1.0, 2.0, 3.0))
        * Matrix4::from_nonuniform_scale(2.0, 2.0, 2.0)
        * Matrix4::from_angle_x(cgmath::Rad(std::f32::consts::FRAC_PI_4))).into()
}

pub fn compose_matrix_to(m: &mut [[f32; 4]; 4], position: &[f32; 3], quaternion: &[f32; 4], scale: &[f32; 3]){
    let result = compose_matrix(position, quaternion, scale);
    m.clone_from(&result)
}
