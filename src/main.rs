mod camera;
mod components;
mod map;
mod mouse;
mod resources;
mod tanks;
mod utils;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use camera::CameraPlugin;
use components::*;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use tanks::TanksPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;

const TANK_SPEED: f32 = 50.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const MAP_SIZE: f32 = 2000.0;
const TANK_COUNT: usize = 100;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TanksPlugin,
            UtilsPlugin,
            CameraPlugin,
            MapPlugin,
            ResourcesPlugin,
            MousePlugin,
            // RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            // WorldInspectorPlugin::new(),
        ))
        .run();
}
