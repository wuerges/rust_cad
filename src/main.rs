
fn main() {

    use rust_cad_geometry::*;
    use rust_cad_geometry::geometry::*;

    let r = rtree::RTree::<i32>::Sent;

    // let empty = geometry::Rect::empty();
    // r.insert(empty, 1);

    // let tree = (1..1000000).fold(r, |t,i| t.insert(empty, i));

    println!("-------------------------------------------------");
    println!("Tree = {:?}", r);
    let r1 = Rect::build_unsafe([897125487, 825057424, 716138779], [3253067062, 2391459330, 3751124909]);
    let r = r.insert(r1, 1);
    println!("-------------------------------------------------");
    println!("Tree = {:?}", r);
    let r2 = Rect::build_unsafe([590069298, 455955083, 409746648], [844108913, 600953719, 502025764]);
    let r = r.insert(r2, 1);
    println!("-------------------------------------------------");
    println!("Tree = {:?}", r);
    let r3 = Rect::build_unsafe([152449100, 19176439, 949041722], [3143509272, 706576841, 1801634347]);
    let r = r.insert(r3, 1);
    println!("-------------------------------------------------");
    println!("Tree = {:?}", r);

    // println!("Rects = {:?} {:?} {:?}", r1, r2, r3);
}