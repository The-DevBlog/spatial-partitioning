mod bevy_rapier;

use bevy_rapier::BevyRapierPlugin;

use bevy::prelude::*;

const TANK_SPEED: f32 = 50.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const MAP_SIZE: f32 = 2000.0;
const TANK_COUNT: usize = 100;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BevyRapierPlugin,
            // WorldInspectorPlugin::new(),
        ))
        .run();
}
