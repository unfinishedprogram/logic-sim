use glam::Vec2;
use util::bounds::Bounds;

use crate::logic::gate::Gate;

pub struct CircuitElement {
    pub gate: Gate,
    pub position: Vec2,
}

impl CircuitElement {
    pub fn hit_test(&self, position: Vec2) -> bool {
        self.bounds().contains(position)
    }

    pub fn bounds(&self) -> Bounds {
        self.gate.bounds().translate(self.position)
    }
}
