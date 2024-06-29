use std::ops::IndexMut;

use crate::util::handle::Handle;

use super::{render_queue::RenderQueue, Frame};

pub struct Response<'a, T> {
    frame: &'a mut Frame,
    handle: Handle<T>,
}

impl Frame {
    pub fn response<T>(&mut self, handle: Handle<T>) -> Response<T> {
        Response {
            frame: self,
            handle,
        }
    }
}

impl<'a, T> Response<'a, T>
where
    RenderQueue: IndexMut<Handle<T>, Output = T>,
{
    pub fn item_mut(&mut self) -> &mut T {
        &mut self.frame.render_queue[self.handle]
    }

    pub fn item(&self) -> &T {
        &self.frame.render_queue[self.handle]
    }
}
