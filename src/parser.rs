use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use crate::geometry::Rect;

#[derive(Debug)]
pub struct Parse {
    pub shapes : Vec<Rect>,
    pub obstacles : Vec<Rect>,
    pub spacing : u32,
    pub via_cost : u32,
    pub boundary : Rect
}

pub fn parse_file(filename : &Path) -> io::Result<Parse> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);

    let mut shapes = Vec::new();
    let mut obstacles  = Vec::new();
    let mut spacing = 0;
    let mut via_cost = 0;
    let mut boundary = Rect::empty();

    for io_line in f.lines() {
        let line = io_line?;
        let tokens : Vec<_> = line.split(' ').collect();

        match tokens[0] {
            "ViaCost" => {
                via_cost = tokens[2].parse().unwrap();
            },
            "Spacing" => {
                spacing = tokens[2].parse().unwrap();
            },
            _ => {
                panic!("parser error. unmatched line: {:?}", line);
            }
        }

        println!("{}", line);
    }
    return Ok(Parse {
        shapes : shapes,
        obstacles : obstacles,
        spacing : spacing,
        via_cost : via_cost,
        boundary : boundary
    });
}

