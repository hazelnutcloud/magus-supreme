use benimator::SpriteSheetAnimation;
use bevy::prelude::*;

// =========================================================
// ====================== RESOURCES ========================
// =========================================================

pub struct SpriteSheets {
    pub wizard: Handle<TextureAtlas>,
}

pub struct Animations {
    pub wizard_idle: Handle<SpriteSheetAnimation>,
}
