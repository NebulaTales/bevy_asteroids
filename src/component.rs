use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct Velocity {
    pub translation: Vec2,
    pub rotation: f32,
}

impl Velocity {
    pub fn new(translation: Vec2, rotation: f32) -> Self {
        Velocity {
            translation,
            rotation,
        }
    }

    pub fn with_translation(x: f32, y: f32) -> Self {
        Velocity {
            translation: Vec2 { x, y },
            ..Default::default()
        }
    }

    pub fn with_rotation(rotation: f32) -> Self {
        Velocity {
            rotation,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct Friction(pub f32);

#[derive(Default, Debug)]
pub struct Acceleration {
    pub forward: f32,
    pub rotation: f32,
}

#[derive(Debug)]
pub struct Thrust {
    pub forward: f32,
    pub backward: f32,
    pub yaw: f32,
}

impl Default for Thrust {
    fn default() -> Self {
        Thrust {
            forward: 1000.0,
            backward: 300.0,
            yaw: 17.0,
        }
    }
}

pub struct PlayerControlled;

pub struct LayerMask(pub u8);
pub struct CollisionMask(pub u8);
pub const PLAYER: u8 = 0b00000001;
pub const OBSTACLE: u8 = 0b00000010;
