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
        match tree {
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

    pub fn pop(&mut self) -> T {
        let mut count = 0;
        loop {
            count += 1;
            let x = self.queue.pop().unwrap();
            
            // match x {
                // None => return None,
                // Some(tree) => {
                    match x {
                        RTree::Sent => {
                        },
                        RTree::Leaf(_, data) => {
                            println!("count = {:?}", count);
                            return *data;
                        },
                        RTree::Child(_, child) => {
                            for c in child {
                                self.push(c);
                            }
                        }
                    }
                // }
            // }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    
    // #[quickcheck]
    // fn prop_height_balanced(v : Vec::<Rect> ) -> bool {

    //     let t = RTree::from_list(v.into_iter().zip(1..).collect::<Vec<(Rect,i32)>>());
    //     return t.height() >= 0;
    // }

    #[quickcheck]
    fn prop_find_elements(v : Vec::<Rect> ) -> bool {

        let v2 = v.clone();

        let t = RTree::from_list(v.into_iter().zip(1..).collect::<Vec<(Rect,i32)>>());

        for r in v2 {
            if ! t.hits(r) {
                return false;
            }
        }
        return true;
    }


    #[quickcheck]
    fn prop_number_elements(center : Rect, rects : Vec::<Rect> ) -> bool {
        let rlen = rects.len();

        let t0 = RTree::empty();
        let t = rects.into_iter().fold(t0, |t,r| t.insert(r, ()));

        let mut qu = RTreeQueue::new(center, &t);

        let mut count = 0;
        loop {
            if qu.is_empty() {
                break;
            }
            let z = qu.pop();
            count += 1;
        }

        return rlen == count;
    }
    
    #[test]
    fn insert_180000() {
        
        use rand::{RngCore};
        let mut rng = rand::thread_rng();
        
        let rects : Vec<_> = (0..180000).map( |_| Rect::build(
            [rng.next_u32()  % 180000, rng.next_u32()  % 180000, rng.next_u32()  % 180000], 
            [rng.next_u32()  % 180000, rng.next_u32()  % 180000, rng.next_u32()  % 180000])).collect();
            
        let t0 = RTree::empty();
        let tree = rects.iter().fold(t0, |t,r| t.insert(*r, r));
        let mut qu = RTreeQueue::new(rects[0], &tree);


        let mut popped = Vec::new();

        loop {
            if qu.is_empty() {
                break;
            }
            let z = qu.pop();
            popped.push(z);
        }

        for i in 1..popped.len() {

            assert!(popped[i].distance(&rects[0]) >= popped[i-1].distance(&rects[0]));

            println!("distance = {:?}", popped[i].distance(&rects[0]));
        }

        assert_eq!(tree.len(), 180000);
    }

}
