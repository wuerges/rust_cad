use crate::geometry::*;
use crate::rtree::*;

pub struct Route { 
    pub length : u32, 
    pub path : Vec<Pt> 
}


pub fn astar(
    u : Rect, 
    v : Rect,
    shape_index :&RTree<usize> , 
    obstacle_index : &RTree<usize>, 
    boundary: Rect) -> Route 
{
    return Route {
        length : u.distance(&v),
        path : Vec::new()
    };
}