use std::collections::HashSet;

use util::bounds::Bounds;

use crate::logic::hit_test::HitTestResult;

use super::connection::{ConnectionIdx, ElementIdx, IOSpecifier};

#[derive(Default)]
pub struct ElementSelection {
    pub(super) elements: HashSet<HitTestResult>,
    pub(super) bound_select: Option<Bounds>,
}

impl ElementSelection {
    pub fn contains(&self, element: HitTestResult) -> bool {
        self.elements.contains(&element)
    }

    pub fn toggle(&mut self, element: HitTestResult) {
        if self.elements.contains(&element) {
            self.elements.remove(&element);
        } else {
            self.elements.insert(element);
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn connections(&self) -> impl IntoIterator<Item = ConnectionIdx> + '_ {
        self.elements.iter().filter_map(|hit| {
            if let HitTestResult::Connection(idx) = hit {
                Some(*idx)
            } else {
                None
            }
        })
    }

    pub fn connection_nodes(&self) -> impl IntoIterator<Item = IOSpecifier> + '_ {
        self.elements.iter().filter_map(|hit| {
            if let HitTestResult::IO(node) = hit {
                Some(*node)
            } else {
                None
            }
        })
    }

    pub fn elements(&self) -> impl IntoIterator<Item = ElementIdx> + '_ {
        self.elements.iter().filter_map(|hit| {
            if let HitTestResult::Element(element) = hit {
                Some(*element)
            } else {
                None
            }
        })
    }
}
