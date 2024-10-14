use bevy::prelude::{Resource, World};
use crate::app2d::app2d::Application2D;
use crate::content::sync::schema::SyncCmd;

pub trait CmdSwap: Send + Sync {
    fn get_cmds(&self, id: u64) -> Vec<SyncCmd>;
}

#[derive(Resource)]
pub struct AppConfig {
    pub id: u64,
    cmd_swap: Box<dyn CmdSwap>,
}

impl AppConfig {
    pub fn new(id: u64, cmd_swap: Box<dyn CmdSwap>) -> Self {
        Self {
            id,
            cmd_swap,
        }
    }

    pub fn get_cmds(&self) -> Vec<SyncCmd> {
        self.cmd_swap.get_cmds(self.id)
    }
    pub fn world_mut(&mut self) -> &mut World {
        let app = unsafe { &mut *(self.id as *mut Application2D) };
        app.world_mut()
    }
}
