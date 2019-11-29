use crate::geometry::Rect;

#[derive(Clone)]
pub enum RTree<T> {
    Sent,
    Leaf  (Rect, T),
    Child (Rect, Vec<Box<RTree<T>>>)
}

enum Ins<T> {
    NoSplit(Box<RTree<T>>),
    Split(Box<RTree<T>>, Box<RTree<T>>)

}

impl<T: Copy> RTree<T> {

    fn children(&mut self) -> Vec<Box<RTree<T>>> {
        match self {
            RTree::<T>::Sent => Vec::new(),
            RTree::<T>::Leaf(_, _) => Vec::new(),
            RTree::<T>::Child(_, child) => child.drain(0..).collect()
        }
    }

    fn bb(& self) -> Rect {
        match self {
            RTree::<T>::Sent => Rect::empty(),
            RTree::<T>::Leaf(key, _) => *key,
            RTree::<T>::Child(bb, _) => *bb
        }
    }
    
    fn split_subtrees(subtrees : &mut Vec<Box<RTree<T>>>) -> Box<RTree<T>> {

        let recs : Vec<Rect> = subtrees.iter().map( |t| t.bb() ).collect();

        let mut r1 = recs[0];
        let mut r2 = recs[1];
        let mut area = 0.0;

        for i in 0..recs.len() {
            for j in 0..recs.len() {
                if i < j {
                    let x = recs[i].mbr(&recs[j]).area();
                    if x > area {
                        area = x;
                        r1 = recs[i];
                        r2 = recs[j];
                    }
                }
            }
        }

        let mut left : Vec<Box<RTree<T>>> = Vec::new();
        let mut right : Vec<Box<RTree<T>>> = Vec::new();

        subtrees.drain(0..).for_each(|t| {
            if r1.mbr(&t.bb()).area() > r2.mbr(&t.bb()).area() {
                left.push(t)
            }
            else {
                right.push(t)
            }
        });

        subtrees.append(&mut left);

        // TODO bugado se left or right forem vazios
        let bb = right.iter().fold(right[0].bb(), |a,b| a.mbr(&b.bb()));

        return Box::new(RTree::<T>::Child(bb, right));
    
    }

    fn insert_node_p(&mut self, r : Rect, v : T ) -> Option<Box<RTree<T>>> {
        match self {
            RTree::<T>::Sent => {
                *self = RTree::<T>::Leaf(r, v);
                return None
            },
            RTree::<T>::Leaf(_, _) => {
                return Some(Box::new(RTree::<T>::Leaf(r, v)))
            },
            RTree::<T>::Child(bb, subtrees) => { // TODO
                subtrees.sort_by(|t1, t2| 
                    {
                        let x : f64 = bb.mbr(&t1.bb()).area();
                        let y : f64 = bb.mbr(&t2.bb()).area();
                        return x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal);
                    }
                );

                match subtrees[0].insert_node_p(r, v) {
                    None => {
                    }
                    Some(extra_head) => {
                        subtrees.push(extra_head)
                    }
                }

                if subtrees.len() > 8 {

                    let extra_head = RTree::split_subtrees(subtrees);
                    return Some(extra_head)
                }
                return None
            }
        }
    }

    fn insert(&mut self, r : Rect, v : T)  {
        match self.insert_node_p(r, v) {
            None => {
            },
            Some(extra_head) => {
                let bb = self.bb().mbr(&extra_head.bb());
                let main_head = Box::new(RTree::<T>::Child(self.bb(), self.children()));
                let children :Vec<Box<RTree<T>>> = vec![main_head, extra_head];
                *self = RTree::<T>::Child(bb, children);
            }
        }
    }


    fn insert_node_p_imut(&self, r : Rect, v : T ) -> Ins<T> {
        match self {
            RTree::Sent => {
                return Ins::NoSplit(Box::new(RTree::Leaf(r, v)));
            },
            RTree::Leaf(key, value) => {
                return Ins::Split(
                    Box::new(RTree::Leaf(r, v)),
                    Box::new(RTree::Leaf(*key, *value)),
                );
            },
            RTree::Child(bb, subtrees) => { // TODO

                let mut cp : Vec<_> = subtrees.iter().collect();

                cp.sort_by(|t1, t2| 
                    {
                        let x : f64 = bb.mbr(&t1.bb()).area();
                        let y : f64 = bb.mbr(&t2.bb()).area();
                        return x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal).reverse();
                    }
                );

                let head = cp.pop().unwrap();

                match head.insert_node_p_imut(r, v) {
                    Ins::NoSplit(no_split) => {
                        cp.push(&no_split);
                        let bb_new = self.bb().mbr(&no_split.bb());
                        let cp_new : Vec<Box<RTree<T>>> = Vec::new();
                        // cp_new.append(cp.drain(0..).map(|e| *e).);
                        return Ins::NoSplit(Box::new(RTree::Child(bb_new, cp_new)));
                    },
                    Ins::Split(one, two) => {
                        return Ins::NoSplit(Box::new(RTree::Leaf(r, v)));
                    }
                }

                // if subtrees.len() > 8 {

                //     let extra_head = RTree::split_subtrees(subtrees);
                //     return Some(extra_head)
                // }
                // return Ins::NoSplit(Box::new(RTree::Leaf(r, v)));
            }
        }
    }

}