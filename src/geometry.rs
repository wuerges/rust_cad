
pub type Pt = [u32; 3];

// #[derive]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Rect {
    p1 : Pt,
    p2 : Pt
}

impl Rect {

    pub fn build(p1 : Pt, p2 :Pt) -> Rect {
        return Rect {
            p1 : minpt(p1, p2),
            p2 : maxpt(p1, p2)
        }
    }

    pub fn empty() -> Rect {
        return Rect::build_unsafe([0, 0,0],[0,0,0]);
    }
    pub fn build_unsafe(p1 : Pt, p2 :Pt) -> Rect {
        return Rect {
            p1 : p1,
            p2 : p2
        }
    }


    fn expand(&self, e :u32) -> Rect {
        let p1n = [self.p1[0]-e, self.p1[1]-e, self.p1[2]];
        let p2n = [self.p2[0]+e, self.p2[1]+e, self.p2[2]];
        return Rect { p1 : p1n, p2 : p2n };
    }

    pub fn area(&self) -> f64 {
        return zip_pt_with(self.p1, self.p2, 
            &|e1,e2| (e1 as f64 - e2 as f64).abs())
            .iter()
            .fold(1.0, |m,i| m*i);
    }

    pub fn distance(&self, other : &Rect) -> u32 {
        panic!("not implemented");
    }

    pub fn intersection(&self, other : &Rect) -> Option<Rect> {

        let p1i = maxpt(self.p1, other.p1);
        let p2i = minpt(self.p2, other.p2);

        for i in 0..2 {
            if p1i[i] > p2i[i] {
                return None
            }
        }

        return Some(Rect::build(p1i, p2i));
    }

    pub fn mbr(&self, other : &Rect) -> Rect {
        return Rect::build_unsafe(minpt(self.p1, other.p1), maxpt(self.p2, other.p2));
    }
}

fn zip_pt_with<U : Copy, V : Copy, T>(p1 : [U; 3], p2 : [V; 3], f : &dyn Fn(U, V) -> T) -> [T; 3] {
    return [f(p1[0], p2[0]), f(p1[1], p2[1]), f(p1[2], p2[2])];
}

fn minpt(p1 : Pt, p2 : Pt) -> Pt {
    return zip_pt_with(p1, p2, &std::cmp::min);
}

fn maxpt(p1 : Pt, p2 : Pt) -> Pt {
    return zip_pt_with(p1, p2, &std::cmp::max);
}



#[cfg(test)]
mod tests {
    use super::*;
    
    use quickcheck::Arbitrary;
    use quickcheck::Gen;
    
    #[derive(Clone, Debug)]
    struct Pt32(pub Pt);
    impl Arbitrary for Pt32 {

        fn arbitrary<G: Gen>(g : &mut G) -> Self {
            let v : [u32; 3]= [g.next_u32(), g.next_u32(),g.next_u32()];
            return Pt32(v);
        }
    
    }
    impl Arbitrary for Rect {

        fn arbitrary<G: Gen>(g : &mut G) -> Self {
            let Pt32(p1) = Pt32::arbitrary(g);
            let Pt32(p2) = Pt32::arbitrary(g);
            return Rect::build(p1, p2);
        }
    
    }

    #[quickcheck]
    fn prop_minpt_is_min(p1 : Pt32, p2: Pt32) -> bool {
        let p1 = p1.0;
        let p2 = p2.0;

        let x = minpt(p1, p2);

        return x[0] <= p1[0] && x[1] <= p1[1] && x[2] <= p1[2] && 
               x[0] <= p2[0] && x[1] <= p2[1] && x[2] <= p2[2] ;
    }

    #[quickcheck]
    fn prop_maxpt_is_max(p1 : Pt32, p2: Pt32) -> bool {
        let p1 = p1.0;
        let p2 = p2.0;

        let x = maxpt(p1, p2);

        return x[0] >= p1[0] && x[1] >= p1[1] && x[2] >= p1[2] && 
               x[0] >= p2[0] && x[1] >= p2[1] && x[2] >= p2[2] ;
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