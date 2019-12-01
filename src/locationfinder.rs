
use crate::geometry::Pt;
use crate::geometry::Rect;
use crate::rtree::*;
use crate::priorityqueue::*;
use crate::rtreequeue::*;
use crate::graph::*;
use crate::muf::*;
use std::rc::Rc;

// struct Edge {
//     u : usize,
//     v : usize,
//     path : Option<Vec<Pt>>
// }

enum Edge {
    RTQ (Box<RTreeQueue<usize>>), // Represents the neighboors of a vertex
    Edge { u : usize, v : usize },
    Route { u: usize, v : usize, length : u32, route : Vec<Pt> }
}

struct Finder {
    shape_index : RTree<usize>,
    obs_index   : RTree<usize>,
    g : Graph<Rect>,
    // shapes      : Vec<Rect>,
    bounds      : Rect,
}


impl Finder {
    fn new ( shapes : Vec<Rect>, obstacles : Vec<Rect>, bounds : Rect) -> Self {

        Finder {
            shape_index : RTree::from_list(shapes.iter().cloned().zip(0..).collect()),
            obs_index   : RTree::from_list(obstacles.into_iter().zip(0..).collect()),
            // shapes      : shapes.clone(),
            g           : Graph::new(shapes),
            bounds      : bounds
        }
    }

    fn route(&mut self) -> Vec<Vec<Pt>> {
        let mut q = PriorityQueue::<u32, usize>::new();
        let mut muf = MUF::new(self.g.vertices.len());
        let mut num_edges = 0;

        let mut edges = Vec::<Edge>::new();

        for (u, u_rect) in self.g.vertices.iter().enumerate() {
            let mut z = RTreeQueue::new(*u_rect, Rc::clone(&self.shape_index.0));

            while z.peek() == 0 {
                let v = z.pop().unwrap();
                num_edges += 1;
                muf.union(u, v);
            }

            q.push(z.peek(), edges.len());
            edges.push(Edge::RTQ(Box::new(z)));
        }

        while num_edges + 1 < self.g.vertices.len() {
            match edges[q.pop().unwrap().value] {
                Edge::RTQ(rtq) => {

                },
                Edge::Edge {u, v} => {

                },
                Edge::Route {u, v, length, route} => {

                }
            }            
        }



        return Vec::new();
    }
}