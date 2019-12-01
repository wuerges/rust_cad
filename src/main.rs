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

    let result = parse_file(Path::new(input));

    println!("Result = {:?}", result);

    let ok = result?;

    let mut f = Finder::new( 
        ok.shapes,
        ok.obstacles,
        ok.boundary
    );

    let route = f.route();

    println!("resulting route: {:?}", route);
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
