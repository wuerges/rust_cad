use crate::rtree::*;
use crate::geometry::Rect;
use crate::priorityqueue::*;
// use std::borrow::BorrowMut;
use std::rc::Rc;


pub struct RTreeQueue<T> {
    center : Rect,
    // rtree : Rc<RTreeImpl<T>>,
    queue : PriorityQueue<u32, Rc<RTreeImpl<T>>>
}

impl<T: Copy> RTreeQueue<T> {

    pub fn new(cent : Rect, qu : Rc<RTreeImpl<T>>) -> Self {
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

    pub fn peek(&self) -> u32 {
        return self.queue.peek().unwrap_or(std::u32::MAX);
    }

    pub fn pop(&mut self) -> Option<T> {
        
        loop {
            let x = self.queue.pop();
            if x.is_none() {
                return None
            }
            else {
                let tree = x.unwrap().value;
                match &*tree {
                    RTreeImpl::Sent => {},
                    RTreeImpl::Leaf(_, data) => {
                        return Some(*data);
                    },
                    RTreeImpl::Child(_, child) => {
                        for c in child.iter().cloned() {
                            self.push(c);
                        }
                    }
                }
            }

            // match x {
            //     None => return None,
            //     Some(tree) => {
            //         match *tree.value {
            //             RTreeImpl::Sent => {
            //             },
            //             RTreeImpl::Leaf(_, data) => {
            //                 return Some(data);
            //             },
            //             RTreeImpl::Child(_, child) => {
            //                 // for c in child.to_iter() {
            //                 //     self.push(Rc::clone(c));
            //                 // }
            //             }
            //         }
            //     }
            // }
        }
    }
}