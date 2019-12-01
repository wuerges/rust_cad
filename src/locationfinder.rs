
use crate::geometry::Pt;
use crate::geometry::Rect;
use crate::graph::*;
use crate::rtree::*;

struct Edge {
    u : usize,
    v : usize,
    path : Option<Vec<Pt>>
}

struct Finder {
    shape_index : RTree<usize>,
    obs_index   : RTree<usize>,
    g : Graph<Rect>,
    bounds      : Rect,
}


impl Finder {
    fn new ( shapes : Vec<Rect>, obstacles : Vec<Rect>, bounds : Rect) -> Self {

        Finder {
            shape_index : RTree::from_list(shapes.iter().cloned().zip(0..).collect()),
            obs_index   : RTree::from_list(obstacles.into_iter().zip(0..).collect()),
            g : Graph::new(shapes),
            bounds : bounds
        }
    }
}