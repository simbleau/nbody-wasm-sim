use glam::Vec2;

use super::Body;

pub struct Collision {
    pub v1_final: Vec2,
    pub v2_final: Vec2,
}

pub fn collides(body: &Body, other: &Body) -> bool {
    // TODO: Determine if 2 circles collide with rapier
    false
}

pub fn get_collision(b1: &Body, b2: &Body) -> Option<Collision> {
    if collides(b1, b2) {
        // TODO: Get collision response with rapier
        Some(Collision {
            v1_final: todo!(),
            v2_final: todo!(),
        })
    } else {
        None
    }
}
