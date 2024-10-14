use bevy::prelude::Component;
use crate::utils::id::UID;

#[derive(Component)]
pub struct ExampleShape(pub UID);
