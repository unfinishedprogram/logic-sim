use glam::Vec2;

pub trait Clickable {
    fn hit_test(&self, position: Vec2) -> bool;
}
