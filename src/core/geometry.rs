use std::marker::PhantomData;
use wgpu::Device;
use wgpu::util::DeviceExt;

struct GeometryAttribute<T> {
    /// 单组数量
    pub item_size: u8,
    /// array.len / item_size
    pub count: usize,
    pub array: Vec<T>,
    /// 需要被更新
    pub need_update: bool,
    /// 这是为了保证类型安全性
    _marker: PhantomData<T>,
    /// 上传至 gpu 的 buffer 对象
    wgpu_buffer: Option<wgpu::Buffer>,
}

impl<T> GeometryAttribute<T>
where
    T: Copy + Clone,
{
    fn new(item_size: u8, array: Vec<T>) -> Self {
        let count = array.len() / item_size as usize;
        GeometryAttribute {
            item_size,
            count,
            array,
            need_update: true,
            _marker: PhantomData,
            wgpu_buffer: None,
        }
    }

    fn update(&mut self, device: &Device) {
        if !self.need_update {
            return;
        }
        if self.wgpu_buffer.is_some() {
            let mut cache = self.wgpu_buffer.take().unwrap();
            cache.destroy();
            drop(cache);
        }
        self.wgpu_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("GeometryAttribute Buffer"),
            contents: bytemuck::cast_slice(&self.array),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }));
        self.need_update = false;
    }
}

#[test]
fn test_new_geometry_attribute() {
    let f32: &[f32] = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let usize: &[usize] = &[1_usize, 2, 3, 4, 5, 6];
    let f64: &[f64] = &[1.0_f64, 2.0, 3.0];
    let attr_f32 = GeometryAttribute::new(3, vec![1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let attr_usize = GeometryAttribute::new(2, vec![1_usize, 2, 3, 4, 5, 6]);
    let attr_f64 = GeometryAttribute::new(1, vec![1.0_f64, 2.0, 3.0]);
    let content_f32: &[u8] = bytemuck::cast_slice(f32);
    assert_eq!(content_f32, bytemuck::cast_slice(&attr_f32.array));
    let content_usize: &[u8] = bytemuck::cast_slice(usize);
    assert_eq!(content_usize, bytemuck::cast_slice(&attr_usize.array));
    let content_f64: &[u8] = bytemuck::cast_slice(f64);
    assert_eq!(content_f64, bytemuck::cast_slice(&attr_f64.array));
}

pub struct GeometryAttributes {
    pub position: GeometryAttribute<f32>,
    pub color: Option<GeometryAttribute<f32>>,
    pub tex_coords: Option<GeometryAttribute<f32>>,
    pub normal: Option<GeometryAttribute<f32>>,
}

struct Geometry {
    pub attributes: GeometryAttributes,
    pub indices: Option<GeometryAttribute<usize>>,
}
