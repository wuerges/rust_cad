use rust_cad_geometry::*;

use locationfinder::*;
use std::env;
use crate::parser::*;
use std::path::Path;
use std::io;
use crate::geometry::Pt;

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
        *s = s.expand(result.spacing);
    }

    result.boundary.p1[2];
    result.boundary.p2[2] = result.metal_layers * result.via_cost;


    let mut f = Finder::new( 
        result.shapes,
        result.obstacles,
        result.boundary
    );

    println!("finder ok");
    
    let route = f.route();
    println!("routes = {:?}", route);

    print_routes(output, route, result.via_cost)?;
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

fn print_routes(outfile : &str, routes : Vec<Vec<Pt>>, via_cost :u32 ) -> io::Result<()> {
    use std::io::*;
    use std::fs::File;
    let mut f = File::create(outfile)?;

    for route in routes {
        for i in 1..route.len() {
            let [x0, y0, z0] = route[i-1];
            let [x1, y1, z1] = route[i];
            
            let l = std::cmp::min(z0, z1) / via_cost;
            let le = std::cmp::max(z0, z1) / via_cost;
            if z0 == z1 {
                let c = if x0 == x1 { 'V' } else { 'H' };
                write!(f, "{}-line M{} ({},{}) ({},{})\n", c, l+1, x0, y0, x1, y1)?;
            }
            else {
                for li in l..le {
                    write!(f, "Via V{} ({},{})\n", li+1, x0, y0)?;
                }
            }
        }
    }


    Ok(())
}