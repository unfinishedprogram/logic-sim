use glam::Vec2;

use crate::logic::gate::Gate;

pub struct CircuitElement {
    pub gate: Gate,
    pub position: Vec2,
}

impl CircuitElement {
    pub fn hit_test(&self, position: Vec2) -> bool {
        self.gate
            .bounds()
            .translate(self.position)
            .contains(position)
    }
}
