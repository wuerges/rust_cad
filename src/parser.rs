use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use crate::geometry::{Rect, Pt};
extern crate regex;

use regex::Regex;

#[derive(Debug)]
pub struct Parse {
    pub shapes : Vec<Rect>,
    pub obstacles : Vec<Rect>,
    pub spacing : u32,
    pub via_cost : u32,
    pub metal_layers : u32,
    pub boundary : Rect
}

pub fn parse_file(filename : &Path) -> io::Result<Parse> {
    let f = File::open(filename)?;
    let f = BufReader::new(f);
    
    let re = regex::Regex::new(r"\((\d+),(\d+)\)").unwrap();

    let mut shapes = Vec::new();
    let mut obstacles  = Vec::new();
    let mut spacing = 0;
    let mut via_cost = 0;
    let mut boundary = Rect::empty();
    let mut metal_layers = 0;

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
            "Boundary" => {
                let p1 = parse_point(tokens[2], &re);
                let p2 = parse_point(tokens[3], &re);
                boundary = Rect::build(p1, p2);
            },
            "#MetalLayers" => {
                metal_layers = tokens[2].parse().unwrap();
            },
            "#RoutedShapes" => {

            },
            "#RoutedVias" => {

            },
            "#Obstacles" => {

            },
            "RoutedShape" => {
                let mut p1 = parse_point(tokens[2], &re);
                let mut p2 = parse_point(tokens[3], &re);
                let l = parse_layer(tokens[1])-1;
                p1[2] = l;
                p2[2] = l;
                shapes.push(Rect::build(p1, p2));
            },
            "RoutedVia" => {
                let mut p1 = parse_point(tokens[2], &re);
                let mut p2 = p1;
                let l = parse_layer(tokens[1])-1;
                p1[2] = l;
                p2[2] = l+1;
                shapes.push(Rect::build(p1, p2));
            },
            "Obstacle" => {
                let mut p1 = parse_point(tokens[2], &re);
                let mut p2 = parse_point(tokens[3], &re);
                let l = parse_layer(tokens[1])-1;
                p1[2] = l;
                p2[2] = l;
                obstacles.push(Rect::build(p1, p2));
            },
            _ => {
                panic!("parser error. unmatched line: {:?}", line);
            }
        }

        // println!("{}", line);
    }
    return Ok(Parse {
        shapes : shapes,
        obstacles : obstacles,
        spacing : spacing,
        via_cost : via_cost,
        metal_layers : metal_layers,
        boundary : boundary
    });
}

fn parse_point(text : &str, re : &Regex) -> Pt {
    let m  = re.captures_iter(text).next();

    let cap = m.unwrap();

    let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();

    return [x, y, 0];
}

fn parse_layer(text : &str) -> u32 {
    return text[1..].parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_regexes() {
        let re = regex::Regex::new(r"\((\d+),(\d+)\)").unwrap();

        

        assert_eq!(parse_point("(7000,3000)", &re), [7000, 3000, 0]);
    }

}