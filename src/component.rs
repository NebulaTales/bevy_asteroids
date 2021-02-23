use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct Velocity {
    pub translation: Vec2,
    pub rotation: f32,
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
    pub yaw: f32,
}

impl Default for Thrust {
    fn default() -> Self {
        Thrust {
            forward: 1000.0,
            yaw: 17.0,
        }
    }
}
