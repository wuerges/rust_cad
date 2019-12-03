use crate::geometry::Rect;

#[derive(Clone,Debug)]
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

    pub fn search<F>(&self, r : &Rect, f : &mut F) -> bool 
    where 
        F: FnMut(&T)-> bool,
    {
        match self {
            RTree::Sent => true,
            RTree::Leaf(key, v) => match r.intersection(&key) {
                None => true,
                Some(_) => f(v)
            }
            RTree::Child(bb, child) => match r.intersection(&bb) {
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
            RTree::Sent => Rect::empty(),
            RTree::Leaf(key, _) => *key,
            RTree::Child(bb, _) => *bb
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
                return 1 + child.iter()
                        .map( |c| c.height())
                        .fold(h1, |h,hi| if h==hi {h} else {-1} )
            }
        }
    }
    

    fn split_subtrees_imut_2(subtrees : Vec<RTree<T>>) -> (RTree<T>, RTree<T>) {

        // Chooses the best rects for the subtrees using O(n^2)
        let recs : Vec<Rect> = subtrees.iter().map( |t| t.bb() ).collect();
        let mut r1 = recs[0];
        let mut r2 = recs[1];
        let mut area = 0.0;

        for i in 0..recs.len() {
            for j in (i+1)..recs.len() {
                let x = recs[i].mbr(&recs[j]).area();
                if x > area {
                    area = x;
                    r1 = recs[i];
                    r2 = recs[j];
                }
            }
        }

        // partitions the node according to the selected rects.
        let mut count = 0;
        let (left, right) : (Vec<_>, Vec<_>)= subtrees.into_iter().partition( |t| {
            
            let a1 = r1.mbr(&t.bb()).area();
            let a2 = r2.mbr(&t.bb()).area();
            
            if a1 == a2 {
                count += 1;
                return count % 2 == 0;
            }
            return a1 < a2;
        });

        let bb1 = left.iter()
            .map(|i| i.bb())
            .fold(left[0].bb(), |sum, i| sum.mbr(&i));
        let bb2 = right.iter()
            .map(|i| i.bb())
            .fold(right[0].bb(), |sum, i| sum.mbr(&i));

        return (RTree::Child(bb1, left), RTree::Child(bb2, right));

    }

    pub fn insert(self, r : Rect, v : T) -> RTree<T> {
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


    fn insert_node_p_imut(self, r : Rect, v : T ) -> Ins<T> {
        match self {
            RTree::Sent => {
                return Ins::NoSplit(RTree::Leaf(r, v));
            },
            RTree::Leaf(key, value) => {
                return Ins::Split(
                    RTree::Leaf(r, v),
                    RTree::Leaf(key, value),
                );
            },
            RTree::Child(bb, mut subtrees) => { 

                subtrees.sort_by(
                    |t1, t2| 
                    {
                        let x : f64 = bb.mbr(&t1.bb()).area();
                        let y : f64 = bb.mbr(&t2.bb()).area();
                        return x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal).reverse();
                    }
                );

                let h = subtrees.pop().unwrap();

                match h.insert_node_p_imut(r, v) {
                    Ins::NoSplit(no_split) => {
                        let new_bb = bb.mbr(&no_split.bb());
                        subtrees.push(no_split);
                        return Ins::NoSplit(RTree::Child(new_bb, subtrees));
                    }
                    Ins::Split(one, two) => {

                        let new_bb = bb.mbr(&one.bb()).mbr(&two.bb());
                        subtrees.push(one);
                        subtrees.push(two);
                        
                        if subtrees.len() < 8 {
                            return Ins::NoSplit(RTree::Child(new_bb, subtrees));
                        }
                        else {
                            let (left, right) = RTree::split_subtrees_imut_2(subtrees);
                            return Ins::Split(left, right);
                        }
                    }
                }

            }
        }
    }

    pub fn empty() -> Self {
        RTree::Sent
    }

    pub fn hits(&self, r : Rect) -> bool {
        let mut res = false;
        self.search(&r, &mut |_| {
            res = true;
            return false;
        });
        return res;
    }
    pub fn from_list(v: Vec<(Rect, T)>) -> Self {
        v.into_iter().fold(RTree::empty(), |t,(r,v)| t.insert(r, v))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[quickcheck]
    fn prop_height_balanced(v : Vec::<Rect> ) -> bool {

        let t = RTree::from_list(v.into_iter().zip(1..).collect::<Vec<(Rect,i32)>>());
        return t.height() >= 0;
    }

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
    fn prop_number_elements(rects : Vec::<Rect> ) -> bool {
        let rlen = rects.len();

        let t0 = RTree::empty();
        let t = rects.into_iter().fold(t0, |t,r| t.insert(r, ()));

        return rlen == t.len();
    }
    
    // #[test]
    // fn insert_180000() {
    //     let t0 = RTree::empty();

    //     let r1 = Rect::build_unsafe([897125487, 825057424, 716138779], [3253067062, 2391459330, 3751124909]);

    //     let tree = (0..180000).fold(t0, |t,i| t.insert(r1, i));

    //     assert_eq!(tree.len(), 180000);
    // }

}
