
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

// enum Edge {
struct RTQ { u : usize, rtq : Box<RTreeQueue<usize>> }
struct Route { u: usize, v : usize, length : u32, route : Vec<Pt> }

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
        let mut muf = MUF::new(self.g.vertices.len());
        let mut num_edges = 0;
        let vertices = &self.g.vertices;
        
        let mut rtqs = Vec::<RTQ>::new();
        let mut routes = Vec::<Route>::new();
        
        let mut rtq_q = PriorityQueue::<u32, usize>::new();
        let mut route_q = PriorityQueue::<u32, usize>::new();

        for (u, u_rect) in vertices.iter().enumerate() {
            let mut z = RTreeQueue::new(*u_rect, Rc::clone(&self.shape_index.0));

            while z.peek() == 0 {
                let v = z.pop().unwrap();
                if muf.find(u) != muf.find(v) {
                    muf.union(u, v);
                    num_edges += 1;
                }
            }

            rtq_q.push(z.peek(), rtqs.len());
            rtqs.push(RTQ{ u : u, rtq : Box::new(z) });
        }

        while num_edges + 1 < self.g.vertices.len() {
            let e = q.pop().unwrap().value;

            match &mut edges[e] {
                Edge::RTQ {u, rtq} => {
                    let v = rtq.pop().unwrap();
                    q.push(rtq.peek(), e);
                    if muf.find(v) != muf.find(*u) {
                        let d = vertices[*u].distance(&vertices[v]);
                        
                        q.push(d, edges.len());
                        edges.push(Edge::Route {u : *u, v: v, length : d, route : Vec::new() });
                    }
                },
                // Edge::Edge {u, v} => {

                // },
                Edge::Route {u, v, length, route} => {

                }
            }            
        }



        return Vec::new();
    }
}