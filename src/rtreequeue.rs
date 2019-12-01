use crate::rtree::*;
use crate::geometry::Rect;
use crate::priorityqueue::*;
use std::rc::Rc;


pub struct RTreeQueue<T> {
    center : Rect,
    // rtree : Rc<RTreeImpl<T>>,
    queue : PrioriteQueue<u32, Rc<RTreeImpl<T>>>
}

impl<T: Copy> RTreeQueue<T> {

    pub fn is_empty(&self) -> bool {
        return self.queue.peek().is_none();
    }

    pub fn push(&mut self, tree: Rc<RTreeImpl<T>>) {
        match *tree {
            RTreeImpl::Sent => {},
            RTreeImpl::Leaf(rect, _) => {
                self.queue.push(rect.distance(&self.center), tree);
            }
            RTreeImpl::Child(rect, _) => {
                self.queue.push(rect.distance(&self.center), tree);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let x = self.queue.pop();

        match x {
            None => {},
            Some(tree) => match &*tree.value {
                RTreeImpl::Sent => {
                    return self.pop()
                },
                RTreeImpl::Leaf(_, data) => {
                    return Some(*data);
                },
                RTreeImpl::Child(_, child) => {
                    for c in child {
                        self.push(Rc::clone(c));
                    }
                    return self.pop();
                }
            }
        };

        return None;
    }

}