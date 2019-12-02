use crate::geometry::Rect;
// use std::rc::Rc;

#[derive(Clone,Debug)]
pub enum RTreeImpl<T> {
    Sent,
    Leaf  (Rect, T),
    Child (Rect, Vec<RTreeImpl<T>>)
}

// enum Ins<'a, T> {
//     NoSplit(RTreeImpl<'a, T>),
//     Split(RTreeImpl<'a, T>, RTreeImpl<'a, T>)
// }

#[derive(Debug)]
pub struct RTree<T> (pub RTreeImpl<T>);

// impl<'a, T:Copy> RTree<'a, T> {
//     pub fn empty() -> Self {
//         RTree(RTreeImpl::Sent)
//     }

//     pub fn hits(&self, r : Rect) -> bool {
//         let mut res = false;
//         self.search(&r, &mut |_| {
//             res = true;
//             return false;
//         });
//         return res;
//     }

//     pub fn from_list(v: Vec<(Rect, T)>) -> Self {
//         return Self::empty();
//         // v.into_iter().fold(RTree::empty(), |t,(r,v)| t.insert(r, v))
//     }

//     pub fn insert(&mut self, r : Rect, v : T) {
//         match self.0.insert_node_p_imut(r, v) {
//             Ins::NoSplit(no_split) => {
//                 RTree(no_split)
//             },
//             Ins::Split(one, two) => {
//                 let bb = one.bb().mbr(&two.bb());
//                 return RTree(RTreeImpl::Child(bb, vec![&one, &two]));
//             }
//         }
//     }

//     pub fn search<F>(&self, r : &Rect, f : &mut F) -> bool 
//     where 
//         F: FnMut(&T)-> bool,
//     {
//         return self.0.search(r, f);
//     }

//     pub fn len(&self) -> usize {
//         self.0.len()
//     }

//     pub fn collect(&self, r : &Rect) -> Vec<T> {
//         return self.0.collect(r);
//     }
// }

impl<T: Copy> RTreeImpl<T> {

    pub fn search<F>(&self, r : &Rect, f : &mut F) -> bool 
    where 
        F: FnMut(&T)-> bool,
    {
        match self {
            RTreeImpl::Sent => true,
            RTreeImpl::Leaf(key, v) => match r.intersection(&key) {
                None => true,
                Some(_) => f(v)
            }
            RTreeImpl::Child(bb, child) => match r.intersection(&bb) {
                None => true,
                Some(_) => {
                    for c in child {
                        if !c.search(r, f) {
                            return false
                        }
                    }
                    return true
                }
            }
        }
    }

    pub fn collect(&self, r : &Rect) -> Vec<T> {
        let mut vals = Vec::<T>::new();

        self.search(r, &mut |t| {
            vals.push(*t);
            return true;
        });
        return vals;
    }

    fn bb(& self) -> Rect {
        match self {
            RTreeImpl::Sent => Rect::empty(),
            RTreeImpl::Leaf(key, _) => *key,
            RTreeImpl::Child(bb, _) => *bb
        }
    }

    pub fn len(&self) -> usize {
        match self {
            RTreeImpl::Sent => 0,
            RTreeImpl::Leaf(_,_) => 1,
            RTreeImpl::Child(_, child) => {
                return child.iter()
                        .fold(0, |sum,t| sum + t.len() );
            }
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            RTreeImpl::Sent => 0,
            RTreeImpl::Leaf(_,_) => 1,
            RTreeImpl::Child(_, child) => {
                let h1 = child[0].height();
                return child.iter()
                        .map( |c| c.height())
                        .fold(h1, |h,hi| if h==hi {h} else {-1} )
            }
        }
    }
    

    fn split_subtrees_imut_2(subtrees : &mut Vec<RTreeImpl<T>>, newsubtrees : &mut Vec<RTreeImpl<T>>) {

        // Chooses the best rects for the subtrees using O(n^2)
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


        // partitions the node according to the selected rects.

        let mut i = 0;
        // let mut count = 0;
        loop {
            let a1 = r1.mbr(&subtrees[i].bb()).area();
            let a2 = r2.mbr(&subtrees[i].bb()).area();
            
            // if a1 == a2 {
                //     return count % 2 == 0;
                // }
             
            if a1 > a2 {
                // count += 1;
                newsubtrees.push(subtrees.swap_remove(i));
            }
            else {
                i += 1;
            }

            if i >= subtrees.len() {
                break;
            }
        }

        // let (left, right) : (Vec<_>, Vec<_>)= subtrees.into_iter().partition( |t| {
            
        //     let a1 = r1.mbr(&t.bb()).area();
        //     let a2 = r2.mbr(&t.bb()).area();
            
        //     if a1 == a2 {
        //         count += 1;
        //         return count % 2 == 0;
        //     }
        //     return a1 < a2;
        // });

        // *subtrees = left;
        // right = right;

        // let bb1 = left.iter()
        //     .map(|i| i.bb())
        //     .fold(left[0].bb(), |sum, i| sum.mbr(&i));
        // let bb2 = right.iter()
        //     .map(|i| i.bb())
        //     .fold(right[0].bb(), |sum, i| sum.mbr(&i));

        // return (RTreeImpl::Child(bb1, *left), RTreeImpl::Child(bb2, *right));

    }

    fn insert_node_p_imut(&mut self, r : Rect, v : T ) -> Option<RTreeImpl<T>> {
        match self {
            RTreeImpl::Sent => {
                *self = RTreeImpl::Leaf(r, v);
                return None;
            },
            RTreeImpl::Leaf(_, _) => {
                return Some(RTreeImpl::Leaf(r, v));
            },
            RTreeImpl::Child(bb, subtrees) => { 

                subtrees.sort_by(
                    |t1, t2| 
                    {
                        let x : f64 = bb.mbr(&t1.bb()).area();
                        let y : f64 = bb.mbr(&t2.bb()).area();
                        return x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal).reverse();
                    }
                );

                let mut h = subtrees.pop().unwrap();
                match h.insert_node_p_imut(r, v) {
                    None => {
                        *bb = h.bb().mbr(bb);
                        subtrees.push(h);
                        return None;
                    },
                    Some(split) => {
                        *bb = split.bb().mbr(&h.bb()).mbr(bb);
                        subtrees.push(h);
                        subtrees.push(split);

                        if subtrees.len() < 8 {
                            return None;
                        }
                        else {
                            let mut right = Vec::new();
                            RTreeImpl::split_subtrees_imut_2(subtrees, &mut right);
                            *bb = subtrees.iter().fold(subtrees[0].bb(), |acc,i| acc.mbr(&i.bb()));
                            let right_bb = right.iter().fold(right[0].bb(), |acc,i| acc.mbr(&i.bb()));
                            return Some(RTreeImpl::Child(right_bb, right));
                        }
                    }
                }
            }
        }
    }

}



// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     #[quickcheck]
//     fn prop_height_balanced(v : Vec::<Rect> ) -> bool {

//         let t = RTree::from_list(v.into_iter().zip(1..).collect::<Vec<(Rect,i32)>>());
//         return t.0.height() >= 0;
//     }

//     #[quickcheck]
//     fn prop_find_elements(v : Vec::<Rect> ) -> bool {

//         let v2 = v.clone();

//         let t = RTree::from_list(v.into_iter().zip(1..).collect::<Vec<(Rect,i32)>>());

//         for r in v2 {
//             if ! t.hits(r) {
//                 return false;
//             }
//         }
//         return true;
//     }


//     #[quickcheck]
//     fn prop_number_elements(rects : Vec::<Rect> ) -> bool {
//         let rlen = rects.len();

//         let t0 = RTree::empty();
//         let t = rects.into_iter().fold(t0, |t,r| t.insert(r, ()));

//         return rlen == t.len();
//     }
    
//     // #[test]
//     // fn insert_180000() {
//     //     let t0 = RTree::empty();

//     //     let r1 = Rect::build_unsafe([897125487, 825057424, 716138779], [3253067062, 2391459330, 3751124909]);

//     //     let tree = (0..180000).fold(t0, |t,i| t.insert(r1, i));

//     //     assert_eq!(tree.len(), 180000);
//     // }

// }
