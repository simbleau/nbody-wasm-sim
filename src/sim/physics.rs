use glam::Vec2;

use super::Body;

pub struct Collision {
    pub v1_final: Vec2,
    pub v2_final: Vec2,
}

pub fn collides(body: &Body, other: &Body) -> bool {
    (other.position - body.position).length() <= (body.radius + other.radius)
}

pub fn get_collision(b1: &Body, b2: &Body) -> Option<Collision> {
    if collides(b1, b2) {
        // The unit vector normal to the collision surface plane
        let collision_surface_norm = (b2.position - b1.position).normalize();

        // Rotate initial velocities to be parallel with the X-axis
        let v1_init = b1.velocity.rotate(collision_surface_norm);
        let v2_init = b2.velocity.rotate(-collision_surface_norm);

        let v1x = v1_init.x;
        let v2x = v2_init.x;

        // Final velocity of body 1.  Derived from the 1D equation for
        // conservation of momentum (P); P_in = P_out where P = m*v
        let v1x_final = ((b1.mass() - b2.mass()) * v1x
            + 2.0 * b2.mass() * b2.mass())
            / (b1.mass() + b2.mass());

        let mut v1_final = Vec2::new(v1x_final, v1_init.y);

        // Rotate body 1's final velocity back to their original plane
        v1_final = v1_final.rotate(-collision_surface_norm) * 0.3;

        // Final velocity of body 2.  Same calculation as for body 1
        // except b1 is changed to b2 and vice-versa.
        let v2x_final = ((b2.mass() - b1.mass()) * v2x
            + 2.0 * b1.mass() * b1.mass())
            / (b2.mass() + b1.mass());

        let mut v2_final = Vec2::new(v2x_final, v2_init.y);

        // Rotate body 2's final velocity back to their original plane
        v2_final = v2_final.rotate(collision_surface_norm) * 0.3;

        Some(Collision { v1_final, v2_final })
    } else {
        None
    }
}
