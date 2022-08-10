use glam::Vec2;

pub trait AsParticle {
    fn as_particle(&self) -> Particle;
}

pub struct Particle {
    position: Vec2,
    mu: f32,
}

impl Particle {
    pub fn new(position: Vec2, mu: f32) -> Self {
        Self { position, mu }
    }
}

pub struct ParticleSet<P: AsParticle> {
    pub particles: Vec<P>,
}

impl<P: AsParticle> ParticleSet<P> {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }
}

impl<P: AsParticle> ParticleSet<P> {
    pub fn get_accelerations(&self) -> Vec<Vec2> {
        let accelerations =
            self.particles.iter().map(P::as_particle).map(|particle1| {
                self.particles.iter().map(P::as_particle).fold(
                    Vec2::ZERO,
                    |acceleration, particle2| {
                        let dir = particle2.position - particle1.position;
                        let mag_2 = dir.length_squared();

                        let grav_acc = if mag_2 != 0.0 {
                            particle2.mu * dir / (mag_2 * mag_2.sqrt())
                        } else {
                            dir
                        };

                        acceleration + grav_acc
                    },
                )
            });

        accelerations.collect()
    }
}
