
type Pt = [u32; 3];

// #[derive]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Rect {
    p1 : Pt,
    p2 : Pt
}

impl Rect {
    fn expand(&self, e :u32) -> Rect {
        let p1n = [self.p1[0]-1, self.p1[1]-1, self.p1[2]];
        let p2n = [self.p2[0]+1, self.p2[1]+1, self.p2[2]];
        return Rect { p1 : p1n, p2 : p2n };
    }
}

fn zip_pt_with(p1 : Pt, p2 : Pt, f : &dyn Fn(u32, u32) -> u32) -> Pt {
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