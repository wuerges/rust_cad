use std::rc::Rc;

fn main() {

    use rust_cad_geometry::*;
    use rust_cad_geometry::geometry::*;

    let t0 = rtree::Facade(Rc::new(rtree::RTree::<i32>::Sent));

    // let empty = geometry::Rect::empty();
    // r.insert(empty, 1);

    // let tree = (1..1000000).fold(r, |t,i| t.insert(empty, i));

    println!("-------------------------------------------------");
    println!("Tree = {:?}", t0);
    let r1 = Rect::build_unsafe([0, 0, 0], [1, 1, 1]);
    let t = t0.insert(r1, 1);

    println!("-------------------------------------------------");
    println!("Tree = {:?}", t);
    let r2 = Rect::build_unsafe([5, 5, 5], [6, 6, 6]);
    let t = t.insert(r2, 2);

    println!("-------------------------------------------------");
    println!("Tree = {:?}", t);
    let r3 = Rect::build_unsafe([3, 3, 3], [4, 4, 4]);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);
    let t = t.insert(r3, 3);

    let tree = (1..180000).fold(t, |ti,i| ti.insert(r1, i));
    
    println!("-------------------------------------------------");
    println!("Tree = {:?}", tree);
    // println!("Collect = {:?}", tree.collect(&r1));
    // println!("Collect = {:?}", tree.collect(&r2));
    // println!("Collect = {:?}", tree.collect(&r3));

    // println!("Rects = {:?} {:?} {:?}", r1, r2, r3);
}