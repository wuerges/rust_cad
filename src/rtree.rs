use crate::geometry::Rect;

#[derive(Clone, Debug)]
pub enum RTree<T> {
    Sent,
    Leaf  (Rect, T),
    Child (Rect, Vec<RTree<T>>)
}

enum Ins<T> {
    NoSplit(RTree<T>),
    Split(RTree<T>, RTree<T>)
}

impl<T: Copy> RTree<T> {

    // fn children(&mut self) -> Vec<Box<RTree<T>>> {
    //     match self {
    //         RTree::<T>::Sent => Vec::new(),
    //         RTree::<T>::Leaf(_, _) => Vec::new(),
    //         RTree::<T>::Child(_, child) => child.drain(0..).collect()
    //     }
    // }

    fn bb(& self) -> Rect {
        match self {
            RTree::<T>::Sent => Rect::empty(),
            RTree::<T>::Leaf(key, _) => *key,
            RTree::<T>::Child(bb, _) => *bb
        }
    }

    pub fn len(&self) -> usize {
        match self {
            RTree::Sent => 0,
            RTree::Leaf(_,_) => 1,
            RTree::Child(_, child) => {
                return child.iter()
                        .fold(0, |sum,t| sum + t.len() );
            }
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            RTree::Sent => 0,
            RTree::Leaf(_,_) => 1,
            RTree::Child(_, child) => {
                let h1 = child[0].height();
                return child.iter()
                        .map( |c| c.height())
                        .fold(h1, |h,hi| if h==hi {h} else {-1} )
            }
        }
    }
    
    // fn split_subtrees(subtrees : &mut Vec<Box<RTree<T>>>) -> Box<RTree<T>> {

    //     let recs : Vec<Rect> = subtrees.iter().map( |t| t.bb() ).collect();

    //     let mut r1 = recs[0];
    //     let mut r2 = recs[1];
    //     let mut area = 0.0;

    //     for i in 0..recs.len() {
    //         for j in 0..recs.len() {
    //             if i < j {
    //                 let x = recs[i].mbr(&recs[j]).area();
    //                 if x > area {
    //                     area = x;
    //                     r1 = recs[i];
    //                     r2 = recs[j];
    //                 }
    //             }
    //         }
    //     }

    //     let mut left : Vec<Box<RTree<T>>> = Vec::new();
    //     let mut right : Vec<Box<RTree<T>>> = Vec::new();

    //     subtrees.drain(0..).for_each(|t| {
    //         if r1.mbr(&t.bb()).area() > r2.mbr(&t.bb()).area() {
    //             left.push(t)
    //         }
    //         else {
    //             right.push(t)
    //         }
    //     });

    //     subtrees.append(&mut left);

    //     // TODO bugado se left or right forem vazios
    //     let bb = right.iter().fold(right[0].bb(), |a,b| a.mbr(&b.bb()));

    //     return Box::new(RTree::<T>::Child(bb, right));
    
    // }

    fn split_subtrees_imut(subtrees : &Vec<RTree<T>>) -> (RTree<T>, RTree<T>) {

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

        let st = subtrees.to_vec();

        let mut eqs = 0;
        let (left, right) : (Vec<RTree<T>>,Vec<RTree<T>>) = st.into_iter().partition( |t| {
            let a1 = r1.mbr(&t.bb()).area();
            let a2 = r2.mbr(&t.bb()).area();

            if a1 == a2 {
                eqs += 1;
                return eqs % 2 == 0;
            }
            return a1 < a2;
        });
        
        
        let bb_right = right.iter().fold(right[0].bb(), |a,b| a.mbr(&b.bb()));
        let bb_left = left.iter().fold(left[0].bb(), |a,b| a.mbr(&b.bb()));

        return ( RTree::Child(bb_left, left)
               , RTree::Child(bb_right, right));
    
    }

    // fn insert_node_p(&mut self, r : Rect, v : T ) -> Option<Box<RTree<T>>> {
    //     match self {
    //         RTree::<T>::Sent => {
    //             *self = RTree::<T>::Leaf(r, v);
    //             return None
    //         },
    //         RTree::<T>::Leaf(_, _) => {
    //             return Some(Box::new(RTree::<T>::Leaf(r, v)))
    //         },
    //         RTree::<T>::Child(bb, subtrees) => { // TODO
    //             subtrees.sort_by(|t1, t2| 
    //                 {
    //                     let x : f64 = bb.mbr(&t1.bb()).area();
    //                     let y : f64 = bb.mbr(&t2.bb()).area();
    //                     return x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal);
    //                 }
    //             );

    //             match subtrees[0].insert_node_p(r, v) {
    //                 None => {
    //                 }
    //                 Some(extra_head) => {
    //                     subtrees.push(extra_head)
    //                 }
    //             }

    //             if subtrees.len() > 8 {

    //                 let extra_head = RTree::split_subtrees(subtrees);
    //                 return Some(extra_head)
    //             }
    //             return None
    //         }
    //     }
    // }

    // fn insert(&mut self, r : Rect, v : T)  {
    //     match self.insert_node_p(r, v) {
    //         None => {
    //         },
    //         Some(extra_head) => {
    //             let bb = self.bb().mbr(&extra_head.bb());
    //             let main_head = Box::new(RTree::<T>::Child(self.bb(), self.children()));
    //             let children :Vec<Box<RTree<T>>> = vec![main_head, extra_head];
    //             *self = RTree::<T>::Child(bb, children);
    //         }
    //     }
    // }



    pub fn insert(& self, r : Rect, v : T) -> RTree<T> {
        match self.insert_node_p_imut(r, v) {
            Ins::NoSplit(no_split) => {
                no_split
            },
            Ins::Split(one, two) => {
                let bb = one.bb().mbr(&two.bb());
                return RTree::Child(bb, vec![one, two]);
            }
        }
    }


    fn insert_node_p_imut(&self, r : Rect, v : T ) -> Ins<T> {
        match self {
            RTree::Sent => {
                return Ins::NoSplit(RTree::Leaf(r, v));
            },
            RTree::Leaf(key, value) => {
                return Ins::Split(
                    RTree::Leaf(r, v),
                    RTree::Leaf(*key, *value),
                );
            },
            RTree::Child(bb, subtrees) => { 

                let mut sts = subtrees.to_vec();
                let (h, mut ead) = sts.split_last_mut().unwrap();
                let mut new_child = ead.to_owned();

                match h.insert_node_p_imut(r, v) {
                    Ins::NoSplit(no_split) => {
                        let new_bb = bb.mbr(&no_split.bb());
                        new_child.push(no_split);
                        return Ins::NoSplit(RTree::Child(new_bb, new_child));
                    }
                    Ins::Split(one, two) => {
                        let new_bb = bb.mbr(&one.bb()).mbr(&two.bb());
                        new_child.push(one);
                        new_child.push(two);
                        return Ins::NoSplit(RTree::Child(new_bb, new_child));
                    }
                }
                // return Ins::NoSplit(RTree::Sent);


                // let mut cp = subtrees.to_vec();
                // cp.sort_by(
                //     |t1, t2| 
                //     {
                //         let x : f64 = bb.mbr(&t1.bb()).area();
                //         let y : f64 = bb.mbr(&t2.bb()).area();
                //         return x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal).reverse();
                //     }
                // );

                // let (h, ead) = cp.split_last().unwrap();
                // // let mut eadv = ead.to_owned();
                // let mut eadv = ead.to_owned();


                // match h.insert_node_p_imut(r, v) {
                //     Ins::NoSplit(no_split) => {
                //         let bb_new = self.bb().mbr(&no_split.bb());
                //         eadv.push(no_split);
                //         // return Ins::NoSplit(RTree::Child(bb_new, cp));
                //         return Ins::NoSplit(RTree::Child(bb_new, eadv));
                //     },
                //     Ins::Split(one, two) => {

                //         eadv.push(one);
                //         eadv.push(two);

                //         if eadv.len() > 8 {
                //             let (a, b) = RTree::split_subtrees_imut(&eadv);
                //             return Ins::Split(a, b);
                //         }
                        
                //         return Ins::NoSplit(RTree::Leaf(r, v));
                //     }
                // }
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    
    use quickcheck::Arbitrary;
    use quickcheck::Gen;
    
    #[derive(Clone)]
    struct VoidRTree(RTree<()>);

    impl Arbitrary for VoidRTree {

        fn arbitrary<G: Gen>(g : &mut G) -> Self {
            let rects : Vec<Rect> = Vec::<Rect>::arbitrary(g);
            
            let r = RTree::Sent;

            let t = rects.into_iter().fold(r, |t,i| t.insert(i, ()));
            return VoidRTree(t);
        }
    
    }

    impl<T: Arbitrary + Copy> Arbitrary for RTree<T> {

        fn arbitrary<G: Gen>(g : &mut G) -> Self {
            let rects = Vec::<(Rect, T)>::arbitrary(g);
            
            let r = RTree::Sent;

            let t = rects.into_iter().fold(r, |t,(r, v)| t.insert(r, v));

            return t;
        }
    
    }
    
    #[quickcheck]
    fn prop_height_balanced(t : RTree<i32> ) -> bool {
        return t.height() >= 0;
    }


    #[quickcheck]
    fn prop_number_elements(rects : Vec::<Rect> ) -> bool {
        let rlen = rects.len();

        let t0 = RTree::Sent;
        let t = rects.into_iter().fold(t0, |t,r| t.insert(r, ()));

        return rlen == t.len();
    }

}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, xsum(2,2));
//     }
// }