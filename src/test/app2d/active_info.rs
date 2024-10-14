use bevy::{prelude::*, utils::HashMap};
use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct ActiveInfo {
    /// TODO 剩余帧数
    ///
    /// 当关闭了自动运行的帧动画之后，场景将仅由鼠标事件驱动更新。由于帧渲染需要由 requestAnimationFrame 驱动
    /// 来保持与浏览器显示刷新的同步，所以鼠标事件不会直接调用 app.update(), 而是重置此待更新的帧数
    pub remaining_frames: u32,
}

impl ActiveInfo {
    pub fn new() -> Self {
        ActiveInfo {
            remaining_frames: 10,
        }
    }
}
