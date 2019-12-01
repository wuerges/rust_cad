use rust_cad_geometry::*;

use geometry::*;

use locationfinder::*;

fn main() {
    let shapes = Vec::new();
    let obstacles = Vec::new();

    let bound = Rect::empty();

    let mut f = Finder::new( 
        shapes,
        obstacles,
        bound
    );

    let route = f.route();

    println!("resulting route: {:?}", route);
    // println!("Collect = {:?}", tree.collect(&r1));
    // println!("Collect = {:?}", tree.collect(&r2));
    // println!("Collect = {:?}", tree.collect(&r3));

    // println!("Rects = {:?} {:?} {:?}", r1, r2, r3);
}