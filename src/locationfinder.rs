
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
        let mut num_edges = 0;
        let vertices = &self.shapes;
        let mut muf = MUF::new(vertices.len());
        
        let mut routes = Vec::<&Vec::<Pt>>::new();
        
        // let mut rtq_q = PriorityQueue::<u32, Box<RTQ>>::new();
        let mut route_q = PriorityQueue::<u32, (usize, usize, Route)>::new();

        // let mut rtq_vec : Vec<_> = vertices.iter().enumerate().map( |(u, u_rect)| {
        //     RTQ{ u : u, rtq : RTreeQueue::new(*u_rect, &self.shape_index)}
        // }).collect();


        let mut count = 0;
        for (u, u_rect) in vertices.iter().enumerate() {
            count += 1;
            let mut pops = 0;

            let mut qu = RTreeQueue::new(*u_rect, &self.shape_index);
            let v = qu.pop();
            let v = qu.pop();
            let v = qu.pop();
            let v = qu.pop();


            // println!("initializing vertices {:?}/{:?} pops = {:?}", count, vertices.len(), pops);

        }

        println!("rtq ok");

        let mut rtq_q = PriorityQueue::<u32, &mut RTQ>::new();
        // let mut count = 0;
        // for q in &mut rtq_vec {

            
        //     count += 1;
            
        //     let mut pops = 0;
        //     while q.rtq.peek() == 0 {
        //         pops += 1;
        //         let v = q.rtq.pop().unwrap();
        //         if muf.find(q.u) != muf.find(v) {
        //             muf.union(q.u, v);
        //             num_edges += 1;
        //         }
        //     }
        //     println!("initializing vertices {:?}/{:?} pops = {:?}", count, vertices.len(), pops);
        //     rtq_q.push(q.rtq.peek(), q);
        // }




        // for (u, u_rect) in vertices.iter().enumerate() {
        //     println!("progress {:?} {:?} {:?}/{:?}", u, u_rect, num_edges+1, vertices.len());

        //     // let z = RTreeQueue::new(*u_rect, Rc::clone(&self.shape_index.0));
        //     // let mut z = ;

        //     let mut q = Box::new(RTQ{ u, rtq : RTreeQueue::new(*u_rect, &self.shape_index) });

        //     while q.rtq.peek() == 0 {

        //         let v = q.rtq.pop().unwrap();
        //         // println!("u={:?}, v={:?}, r_u={:?}, r_v={:?} dist={:?}", u, v, vertices[u], vertices[v], vertices[u].distance(&vertices[v]));
        //         if muf.find(u) != muf.find(v) {
        //             muf.union(u, v);
        //             num_edges += 1;
        //         }
        //     }

        //     let q_key = q.rtq.peek();
        //     rtq_q.push(q_key, q);
        // }

        // while num_edges + 1 < vertices.len() {
        //     if num_edges % 100 == 0 {
        //         println!("progress {:?}/{:?}", num_edges+1, vertices.len());
        //     }
        //     let route_min = route_q.peek().unwrap_or(std::u32::MAX);
        //     if rtq_q.peek().unwrap_or(std::u32::MAX) 
        //     <  route_min {

        //         rtq_q.look( &mut |it| {
        //             let u = it.value.u;

        //             match it.value.rtq.pop() {
        //                 None => {
        //                     return false
        //                 },
        //                 Some(v) => {
        //                     let p = astar(vertices[u], vertices[v], &self.shape_index, &self.obs_index, self.bounds);
        
        //                     if muf.find(u) != muf.find(v) {
        //                         route_q.push(p.length, (u, v, p));
        //                     }
        //                     return true;
        //                 }
        //             }


        //         });

        //     }
        //     else {
        //         match route_q.pop() {
        //             None => {
        //                 break
        //             },
        //             Some( (u, v, route) ) => {
        //                 if muf.find(*u) != muf.find(*v) {
        //                     muf.union(*u, *v);
        //                     routes.push(&route.path);
        //                     num_edges += 1;
        //                 }
        //             }
        //         }
        //     }
        // }

        // return routes;
        return Vec::new();
    }
}