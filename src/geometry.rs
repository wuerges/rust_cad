
pub type Pt = [u32; 3];

// #[derive]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Rect {
    pub p1 : Pt,
    pub p2 : Pt
}

fn dist1d(a :u32, b:u32, aw:u32, bw: u32) -> u32 {

    if a < b {
        if a + aw < b {
            return b - a - aw;
        }
        return 0;
    }
    else {
        if b + bw < a {
            return a - b - bw;
        }
        return 0;
    }
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

    pub fn area(&self) -> i64 {

        // let mut area = 1;
        // for i in 0..2 {
        //     area *= (self.p1[i] as i64 - (1+self.p2[i]) as i64).abs();
        // }
        // return area;


        return zip_pt_with(self.p1, self.p2, 
            &|e1,e2| std::cmp::max(1, (e1 as i64 - e2 as i64).abs()))
            .iter()
            .fold(1, |m,i| m*i);
    }

    pub fn distance(&self, other : &Rect) -> u32 {

        let mut sum : u32 = 0;

        for i in 0..3 {

            let diff_self = if self.p1[i] > self.p2[i] { 
                self.p1[i] - self.p2[i] 
            } 
            else { 
                self.p2[i] - self.p1[i] 
            };

            let diff_other = if other.p1[i] > other.p2[i] { 
                other.p1[i] - other.p2[i] 
            } 
            else { 
                other.p2[i] - other.p1[i] 
            };

            sum += dist1d(self.p1[i], other.p1[i], diff_self, diff_other);
        }

        return sum;
    }

    pub fn intersection(&self, other : &Rect) -> Option<Rect> {

        let p1i = maxpt(self.p1, other.p1);
        let p2i = minpt(self.p2, other.p2);

        for i in 0..3 {
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
            let v : [u32; 3]= [g.next_u32() % 180000, g.next_u32() % 180000,g.next_u32() % 180000];
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


    #[test] 
    fn test_some_distances() {
        let r1 = Rect::build([0,0,1], [1,1,1]);
        let r2 = Rect::build([4,0,2], [5,2,2]);
        let r3 = Rect::build([3,3,3], [5,8,3]);

        println!("distance {:?} {:?} = {:?}", r1, r2, r1.distance(&r2));
        println!("distance {:?} {:?} = {:?}", r1, r3, r1.distance(&r3));
        println!("distance {:?} {:?} = {:?}", r2, r3, r2.distance(&r3));

        println!("area {:?} = {:?}", r1, r1.area());
        println!("area {:?} = {:?}", r2, r2.area());
        println!("area {:?} = {:?}", r3, r3.area());
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