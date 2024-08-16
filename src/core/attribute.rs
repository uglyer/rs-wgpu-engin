use std::marker::PhantomData;

pub type AttributeF32 = Attribute<f32>;
pub type AttributeF64 = Attribute<f64>;
pub type AttributeUsize = Attribute<usize>;

pub struct Attribute<T> {
    /// 单组数量
    pub item_size: u8,
    /// array.len / item_size
    pub count: usize,
    pub data: Vec<T>,
    /// 需要被更新
    pub need_update: bool,
    /// 这是为了保证类型安全性
    _marker: PhantomData<T>,
    /// 上传至 gpu 的 buffer 对象
    wgpu_buffer: Option<wgpu::Buffer>,
}

impl<T> Attribute<T>
where
    T: Copy + Clone,
{
    fn new(item_size: u8, data: Vec<T>) -> Self {
        let count = data.len() / item_size as usize;
        Attribute {
            item_size,
            count,
            data,
            need_update: true,
            wgpu_buffer: None,
            _marker: PhantomData,
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn borrow_data(&self) -> &Vec<T> {
        &self.data
    }
}

#[test]
fn test_new_geometry_attribute() {
    let f32: &[f32] = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let usize: &[usize] = &[1_usize, 2, 3, 4, 5, 6];
    let f64: &[f64] = &[1.0_f64, 2.0, 3.0];
    let attr_f32 = AttributeF32::new(3, vec![1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let attr_usize = AttributeUsize::new(2, vec![1_usize, 2, 3, 4, 5, 6]);
    let attr_f64 = AttributeF64::new(1, vec![1.0_f64, 2.0, 3.0]);
    let content_f32: &[u8] = bytemuck::cast_slice(f32);
    assert_eq!(content_f32, bytemuck::cast_slice(&attr_f32.array));
    let content_usize: &[u8] = bytemuck::cast_slice(usize);
    assert_eq!(content_usize, bytemuck::cast_slice(&attr_usize.array));
    let content_f64: &[u8] = bytemuck::cast_slice(f64);
    assert_eq!(content_f64, bytemuck::cast_slice(&attr_f64.array));
}
