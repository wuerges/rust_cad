use crate::rtree::*;
use crate::geometry::Rect;
use std::collections::BinaryHeap;
use std::rc::Rc;




pub struct RTreeQueue<T> {
    center : Rect,
    rtree : Rc<RTree<T>>,
    queue : BinaryHeap<Rc<RTree<T>>>
}

impl<T> RTreeQueue<T> {
    pub fn is_empty(&self) -> bool {
        return self.queue.is_empty()
    }


}