mod components;
mod plugins;
mod resources;

use bevy::prelude::*;
use plugins::{Loader, Setup};

// =========================================================
// ======================= LABELS ==========================
// =========================================================
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabels {
    Load,
    Spawn,
}

// =========================================================
// ========================= APP ===========================
// =========================================================

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin::default())
        .add_plugin(Loader)
        .add_plugin(Setup)
        .run();
}
