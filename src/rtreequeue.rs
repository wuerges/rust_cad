use crate::rtree::*;
use crate::geometry::Rect;
use crate::priorityqueue::*;


pub struct RTreeQueue<'a, T> {
    center : Rect,
    // rtree : Rc<RTreeImpl<T>>,
    queue : PriorityQueue<u32, &'a RTree<T>>
}

impl<'a, T: Copy> RTreeQueue<'a, T> {

    pub fn new(cent : Rect, qu : &'a RTree<T>) -> Self {
        let mut q = PriorityQueue::new();
        q.push(0, qu);
        return RTreeQueue {
            center : cent,
            queue : q
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.queue.peek().is_none();
    }

    pub fn push(&mut self, tree: &'a RTree<T>) {
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

    pub fn peek(&self) -> u32 {
        return self.queue.peek().unwrap_or(std::u32::MAX);
    }

    pub fn pop(&mut self) -> Option<T> {
        
        loop {
            let x = self.queue.pop();

            match x {
                None => return None,
                Some(tree) => {
                    match tree.value {
                        RTree::Sent => {
                        },
                        RTree::Leaf(_, data) => {
                            return Some(*data);
                        },
                        RTree::Child(_, child) => {
                            for c in child {
                                self.push(c);
                            }
                        }
                    }
                }
            }
        }
    }
}