use crate::app2d::app2d::{Application2D};
use crate::app2d::schema::App2DOptions;
use bevy::app::PluginsState;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::content::sync::config::{AppConfig, CmdSwap};
use crate::content::sync::schema::{SyncCmd, SyncMethod};
use crate::utils::id::UID;
use wasm_bindgen::prelude::*;
use js_sys::BigInt;
use bevy::utils::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn __reality_renderer_nt_warp_call__(id: u64, method: SyncMethod, arg: String) -> String;
}

// 反序列化为Rust结构体
struct CmdSwapJsImplementation;

impl CmdSwap for CmdSwapJsImplementation {
    fn get_cmds(&self, id: u64) -> Vec<SyncCmd> {
        let j = __reality_renderer_nt_warp_call__(id, SyncMethod::GetCmd, "".to_string());
        if j.len() == 0 {
            return vec!();
        }
        return serde_json::from_str(&j).unwrap();
    }
}

#[wasm_bindgen]
pub fn new_app2d(options: String) -> u64 {
    let implementation = CmdSwapJsImplementation;
    let options: App2DOptions = serde_json::from_str(&options).unwrap();
    let app = Application2D::new(options);
    // 包装成无生命周期的指针
    let appid = Box::into_raw(Box::new(app)) as u64;
    let app = unsafe { &mut *(appid as *mut Application2D) };
    app.insert_resource(AppConfig::new(appid, Box::new(implementation)));
    // 返回指针
    appid
}

// /// 是否已完成插件初始化
// /// 初始化未完成之前不能调用帧绘制
// // #[wasm_bindgen]
// pub fn is_preparation_completed(ptr: u64) -> u32 {
//     // 将 window 对象直接存到 app 上, 避免后继查询
//     // let mut windows_system_state: SystemState<Query<(Entity, &Window)>> =
//     //     SystemState::from_world(app.world_mut());
//     // let (entity, _) = windows_system_state.get(app.world_mut()).single();
//     // app.window = entity;
//     0
// }

/// 运行事件循环
#[wasm_bindgen]
pub fn run_app2d(ptr: u64) {
    // 将指针转换为其指代的实际 Rust 对象，同时也拿回此对象的内存管理权
    let app = unsafe { &mut *(ptr as *mut Application2D) };
    app.run();
}

/// 将 js 数组转换为 rust HashMap
fn to_map(arr: js_sys::Array) -> HashMap<Entity, u64> {
    let mut map: HashMap<Entity, u64> = HashMap::new();
    let length = arr.length();
    for i in 0..length {
        let value = bigint_to_u64(arr.get(i));
        if let Ok(v) = value {
            let entity = Entity::from_bits(v);
            map.insert(entity, v);
        }
    }
    map
}

/// 将 js BigInt 转换成 rust u64
/// 测试了几种方式，只有下边的能方式转换成功
fn bigint_to_u64(value: JsValue) -> Result<u64, JsValue> {
    if let Ok(big_int) = BigInt::new(&value) {
        // 转换为字符串，基数为10
        let big_int_str = big_int.to_string(10).unwrap().as_string();
        let big_int_u64: Result<u64, _> = big_int_str.unwrap().parse::<u64>();
        if let Ok(number) = big_int_u64 {
            return Ok(number);
        }
    }
    Err(JsValue::from_str("Value is not a valid u64"))
}
