pub mod camera;
pub mod components;
pub mod map;
pub mod mouse;
pub mod resources;
pub mod tanks;
pub mod utils;

use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use camera::CameraPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use tanks::TanksPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;

pub struct BevyRapierPlugin;

impl Plugin for BevyRapierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TanksPlugin,
            UtilsPlugin,
            CameraPlugin,
            MapPlugin,
            ResourcesPlugin,
            MousePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
        ));
    }
}
