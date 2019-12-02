use rust_cad_geometry::*;

use geometry::*;

use locationfinder::*;
use std::env;
use crate::parser::*;
use std::path::Path;
use std::io;

fn main() -> io::Result<()>  {

    let args: Vec<String> = env::args().collect();
    let (input, output) = parse_config(&args);

    let mut result = parse_file(Path::new(input))?;

    // println!("Result = {:?}", result);

    println!("parse ok");

    for s in &mut result.shapes {
        s.p1[2] *= result.via_cost;
        s.p2[2] *= result.via_cost;
    }

    for s in &mut result.obstacles {
        s.p1[2] *= result.via_cost;
        s.p2[2] *= result.via_cost;
    }

    result.boundary.p1[2] *= result.via_cost;
    result.boundary.p2[2] *= result.via_cost;


    let mut f = Finder::new( 
        result.shapes,
        result.obstacles,
        result.boundary
    );

    println!("finder ok");

    let route = f.route();

    println!("route ok: {:?}", route);
    println!("need to write to file {:?}", output);
    // println!("Collect = {:?}", tree.collect(&r1));
    // println!("Collect = {:?}", tree.collect(&r2));
    // println!("Collect = {:?}", tree.collect(&r3));

    // println!("Rects = {:?} {:?} {:?}", r1, r2, r3);

    return Ok(())
}


fn parse_config(args: &[String]) -> (&str, &str) {
    let input = &args[1];
    let output = &args[2];

    (input, output)
}
