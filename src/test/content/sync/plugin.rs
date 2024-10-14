use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::utils::info;
use crate::content::sync::config::AppConfig;
use crate::content::sync::schema::SyncCmdType;
use crate::content::sync::state::StateMachine;

/// 状态同步插件
#[derive(Default)]
pub struct SyncPlugin;

impl Plugin for SyncPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StateMachine>()
            .add_systems(Update, on_process_sync_cmd);
    }
}

fn on_process_sync_cmd(
    mut windows: Query<&mut Window>,
    app_config: ResMut<AppConfig>,
    // mut state: ResMut<StateMachine>,
    mut app_exit_events: ResMut<Events<AppExit>>
) {
    let cmds = app_config.get_cmds();
    for cmd in cmds {
        match cmd.t {
            SyncCmdType::Resize => {
                let mut window = windows.single_mut();
                let size = cmd.as_vec2();
                window.resolution.set(size.x, size.y);
            }
            SyncCmdType::Destroy => {
                app_exit_events.send(AppExit::Success);
            }
            _ => {
                error!("unknown sync cmd type: {:?}", cmd)
            }
        }
    }
    // world.run_system_once();
}
