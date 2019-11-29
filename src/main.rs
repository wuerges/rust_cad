
fn main() {

    use rust_cad_geometry::*;
    use rust_cad_geometry::geometry::*;

    let r = rtree::RTree::<i32>::Sent;

    // let empty = geometry::Rect::empty();
    // r.insert(empty, 1);

    // let tree = (1..1000000).fold(r, |t,i| t.insert(empty, i));

    println!("-------------------------------------------------");
    println!("Tree = {:?}", r);
    let r1 = Rect::build_unsafe([0, 0, 0], [1, 1, 1]);
    let r = r.insert(r1, 1);
    println!("-------------------------------------------------");
    println!("Tree = {:?}", r);
    let r2 = Rect::build_unsafe([5, 5, 5], [6, 6, 6]);
    let r = r.insert(r2, 2);
    println!("-------------------------------------------------");
    println!("Tree = {:?}", r);
    let r3 = Rect::build_unsafe([3, 3, 3], [4, 4, 4]);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);
    let r = r.insert(r3, 3);

    let tree = (1..180000).fold(r, |t,i| t.insert(r1, i));
    
    println!("-------------------------------------------------");
    println!("Tree = {:?}", tree);
    println!("Collect = {:?}", tree.collect(&r1));
    println!("Collect = {:?}", tree.collect(&r2));
    println!("Collect = {:?}", tree.collect(&r3));

    // println!("Rects = {:?} {:?} {:?}", r1, r2, r3);
}