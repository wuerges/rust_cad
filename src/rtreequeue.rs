use crate::rtree::*;
use crate::geometry::Rect;
use crate::priorityqueue::*;
use std::rc::Rc;


pub struct RTreeQueue<T> {
    center : Rect,
    // rtree : Rc<RTree<T>>,
    queue : PrioriteQueue<u32, Rc<RTree<T>>>
}

impl<T: Copy> RTreeQueue<T> {

    pub fn is_empty(&self) -> bool {
        return self.queue.peek().is_none();
    }

    pub fn push(&mut self, tree: Rc<RTree<T>>) {
        match *tree {
            RTree::Sent => {},
            RTree::Leaf(rect, _) => {
                self.queue.push(rect.distance(&self.center), tree);
            }
            RTree::Child(rect, _) => {
                self.queue.push(rect.distance(&self.center), tree);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let x = self.queue.pop();

        match x {
            None => {},
            Some(tree) => match &*tree.value {
                RTree::Sent => {
                    return self.pop()
                },
                RTree::Leaf(_, data) => {
                    return Some(*data);
                },
                RTree::Child(_, child) => {
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