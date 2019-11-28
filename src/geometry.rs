
type Pt = [i32; 3];

struct Rect {
    p1 : Pt,
    p2 : Pt
}

fn minpt(p1 : Pt, p2 : Pt) -> Pt {
    let mut r : [i32; 3] = p1;
    for i in 0..2 {
        if p2[i] < p1[i] {
            r[i] = p2[i];
        }
    }
    return r;
}

fn maxpt(p1 : Pt, p2 : Pt) -> Pt {
    let mut r : [i32; 3] = p1;
    for i in 0..2 {
        if p2[i] > p1[i] {
            r[i] = p2[i];
        }
    }
    return r;
}


fn xsum(a : i32, b :i32) -> i32 {
    return a + b
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, xsum(2,2));
    }
}