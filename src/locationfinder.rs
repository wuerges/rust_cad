
use crate::geometry::Pt;
use crate::geometry::Rect;
use crate::rtree::*;
use crate::priorityqueue::*;
use crate::rtreequeue::*;
use crate::muf::*;
use crate::astar::*;

struct RTQ<'a> { u : usize, rtq : RTreeQueue<'a, usize> }

pub struct Finder {
    shape_index : RTree<usize>,
    obs_index   : RTree<usize>,
    shapes      : Vec<Rect>,
    bounds      : Rect,
}


impl Finder {
    pub fn new ( shapes : Vec<Rect>, obstacles : Vec<Rect>, bounds : Rect) -> Self {

        Finder {
            shape_index : RTree::from_list(shapes.iter().cloned().zip(0..).collect()),
            obs_index   : RTree::from_list(obstacles.into_iter().zip(0..).collect()),
            shapes      : shapes.clone(),
            bounds      : bounds
        }
    }

    pub fn route(&mut self) -> Vec<Vec<Pt>> {

        // println!("tree: {}", svg);

        let mut num_edges = 0;
        let vertices = &self.shapes;
        let mut muf = MUF::new(vertices.len());
        
        let mut routes = Vec::<Vec::<Pt>>::new();
        
        // let mut rtq_q = PriorityQueue::<u32, Box<RTQ>>::new();
        let mut route_q = PriorityQueue::<u32, (usize, usize, Route)>::new();

        let mut rtq_vec : Vec<_> = vertices.iter().enumerate().map( |(u, u_rect)| {
            RTQ{ u : u, rtq : RTreeQueue::new(*u_rect, &self.shape_index)}
        }).collect();


        println!("rtq ok");

        println!("index size = {:?}   index height = {:?}   max height = {:?}", self.shapes.len(), self.shape_index.height(), self.shape_index.max_height());

        let mut rtq_q = PriorityQueue::<u32, &mut RTQ>::new();
        
        let mut count = 0;

        for q in &mut rtq_vec {
            
            count += 1;
            let mut pops = 0;

            while q.rtq.peek() == 0 {
                pops += 1;
                // q.rtq.push( &self.shape_index)
                let v = q.rtq.pop();
                // let v = &1;
                if muf.find(q.u) != muf.find(v) {
                    muf.union(q.u, v);
                    num_edges += 1;
                }
            }
            if count % 100 == 0 {
                println!("initializing vertices {:?}/{:?} pops = {:?}", count, vertices.len(), pops);
            }
            rtq_q.push(q.rtq.peek(), q);
        }

        while num_edges + 1 < vertices.len() {
            if num_edges % 100 == 0 {
                println!("progress {:?}/{:?}", num_edges+1, vertices.len());
            }
            let route_min = route_q.peek().unwrap_or(std::u32::MAX);
            if rtq_q.peek().unwrap_or(std::u32::MAX) 
            <  route_min {

                rtq_q.look( &mut |it| {
                    let u = it.value.u;

                    if it.value.rtq.is_empty() {
                        return false;
                    }

                    let v = it.value.rtq.pop();
                    let p = astar(vertices[u], vertices[v], &self.obs_index, self.bounds);

                    if muf.find(u) != muf.find(v) {
                        route_q.push(p.length, (u, v, p));
                    }
                    return true;
                });

            }
            else {
                match route_q.pop() {
                    None => {
                        break
                    },
                    Some( (u, v, route) ) => {
                        if muf.find(u) != muf.find(v) {
                            muf.union(u, v);
                            routes.push(route.path);
                            num_edges += 1;
                        }
                    }
                }
            }
        }

        return routes;
    }
}