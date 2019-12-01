
use crate::geometry::Pt;
use crate::geometry::Rect;
use crate::rtree::*;
use crate::priorityqueue::*;
use crate::rtreequeue::*;
use crate::muf::*;
use std::rc::Rc;

struct RTQ { u : usize, rtq : RTreeQueue<usize> }
struct Route { u: usize, v : usize, length : u32, path : Vec<Pt> }

struct Finder {
    shape_index : RTree<usize>,
    obs_index   : RTree<usize>,
    shapes      : Vec<Rect>,
    bounds      : Rect,
}


impl Finder {
    fn new ( shapes : Vec<Rect>, obstacles : Vec<Rect>, bounds : Rect) -> Self {

        Finder {
            shape_index : RTree::from_list(shapes.iter().cloned().zip(0..).collect()),
            obs_index   : RTree::from_list(obstacles.into_iter().zip(0..).collect()),
            shapes      : shapes.clone(),
            bounds      : bounds
        }
    }

    fn route(&mut self) -> Vec<Vec<Pt>> {
        let mut num_edges = 0;
        let vertices = &self.shapes;
        let mut muf = MUF::new(vertices.len());
        
        let mut routes = Vec::<Vec::<Pt>>::new();
        
        let mut rtq_q = PriorityQueue::<u32, Box<RTQ>>::new();
        let mut route_q = PriorityQueue::<u32, Route>::new();

        for (u, u_rect) in vertices.iter().enumerate() {
            let mut z = RTreeQueue::new(*u_rect, Rc::clone(&self.shape_index.0));

            while z.peek() == 0 {
                let v = z.pop().unwrap();
                if muf.find(u) != muf.find(v) {
                    muf.union(u, v);
                    num_edges += 1;
                }
            }

            rtq_q.push(z.peek(), Box::new(RTQ{ u, rtq : z }));
        }

        while num_edges + 1 < vertices.len() {
            if rtq_q.peek().unwrap_or(std::u32::MAX) 
            < route_q.peek().unwrap_or(std::u32::MAX) {

                rtq_q.look( &mut |it| {
                    let v = it.value.rtq.pop().unwrap();

                    route_q.push(0, Route{ u: it.value.u, v:v, length : 0, path : Vec::new() });
                });

            }
            else {
                let e = route_q.pop().unwrap().value;
                if muf.find(e.u) != muf.find(e.v) {
                    muf.union(e.u, e.v);
                    routes.push(e.path);
                    num_edges += 1;
                }
            }
        }

        return routes;
    }
}