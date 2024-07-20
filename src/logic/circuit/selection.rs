use std::collections::HashSet;

use util::bounds::Bounds;

use super::connection::ElementIdx;

#[derive(Default)]
pub struct ElementSelection {
    pub(super) elements: HashSet<ElementIdx>,
    pub(super) bound_select: Option<Bounds>,
}

impl ElementSelection {
    pub fn contains(&self, element: ElementIdx) -> bool {
        self.elements.contains(&element)
    }

    pub fn toggle(&mut self, element: ElementIdx) {
        if self.elements.contains(&element) {
            self.elements.remove(&element);
        } else {
            self.elements.insert(element);
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}
