/// shader 代码生成
/// 提取 `// #include <*>` 内容, 完成字符串模板拼接

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

static HAS_INIT: Mutex<bool> = Mutex::new(false);

lazy_static! {
    static ref SHADER_CODE_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn init() {
    if *HAS_INIT.lock().unwrap() {
        return;
    }
    *HAS_INIT.lock().unwrap() = true;
    let mut map = SHADER_CODE_MAP.lock().unwrap();
    map.insert("copyright".into(), include_str!("assets/copyright.wgsl").into());
    map.insert("struct_vertex_input".into(), include_str!("assets/struct/struct_vertex_input.wgsl").into());
    map.insert("basic".into(), include_str!("assets/basic.wgsl").into());
    map.insert("test".into(), include_str!("assets/test.wgsl").into());
}

// 提取 `// #include <*>` 内容, 完成字符串模板拼接
fn replace_include(s: &str, map: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut start = 0;

    while let Some(match_start) = s[start..].find("// #include <") {
        result.push_str(&s[start..match_start]);

        let match_end = s.find('>').unwrap();
        let include_name = &s[(match_start + 13)..match_end];
        let replacement = map.get(include_name.into());
        match replacement {
            Some(code) => {
                result.push_str(&s[match_start..(match_end + 1)]);
                result.push_str(" --start\n");
                result.push_str(&replace_include(&code, map));
                result.push_str(&s[match_start..(match_end + 1)]);
                result.push_str(" --end\n\n");
            },
            None => {
                result.push_str("include <");
                result.push_str(include_name);
                result.push_str("> --error\n\n");
            },
        }
        start = match_end + 1;
    }

    result.push_str(&s[start..]);
    result
}

pub fn get_shader_code(name: &str) -> Option<String> {
    init();
    let binding = SHADER_CODE_MAP.lock().unwrap();
    let result = binding.get(name);
    match result {
        Some(code) => Some(replace_include(code, &binding)),
        None => None,
    }
}

#[test]
fn test_shader_builder() {
    let code = get_shader_code("basic".into()).unwrap();
    println!("{}", code);
}
