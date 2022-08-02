pub const WORLD_RADIUS: f32 = 50.0;
pub const WORLD_EDGE_SEGMENTS: u32 = 500;

mod body;
pub use body::Body;

mod state;
pub use state::State;

pub mod input;
