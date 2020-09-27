use crate::geometry::*;
use crate::rtree::*;
use crate::priorityqueue::*;
use std::collections::HashMap;

pub struct Route {
    pub length : u32,
    pub path : Vec<Pt>
}

#[derive(Hash,Eq,PartialEq, Debug, Copy, Clone)]
struct Index([usize; 3]);

type Coords = [Vec<u32>; 3];

pub struct AStar<'a> {
    obstacle_index : &'a  RTree<usize>,
    boundary : Rect,
    source : Rect,
    target : Rect,
    coords : Coords
}

impl<'a> AStar<'a> {

    pub fn new(obstacle_index : &'a RTree<usize>, s :Rect, t:Rect, bound : Rect ) -> Self {
        let mut a = AStar {
            obstacle_index : obstacle_index,
            boundary : bound,
            source : s,
            target : t,
            coords : Coords::default(),
        };
        // println!("Creating AStar with {:?} and {:?}", s, t);
        a.add_shape(s);
        a.add_shape(t); 

        let window = s.mbr(&t).expand(crate::config::WINDOW_SIZE);

        obstacle_index.search_with_key(&window, &mut |key,_| {
            a.add_shape(key.expand(1));
            return true;
        });

        a.fix_coords();
        return a;
    }

    fn add_shape(&mut self, r : Rect) {
        for i in 0..3 {
            self.coords[i].push(r.p1[i]);
            self.coords[i].push(r.p2[i]);
        }
    }

    fn fix_coords(& mut self) {
        // println!("coords before t = {:?}", self.coords);
        // println!("boundary = {:?}", self.boundary);

        for i in 0..3 {
            self.coords[i].sort_unstable();
            self.coords[i].dedup();
            let x_min = self.boundary.p1[i];
            let x_max = self.boundary.p2[i];
            self.coords[i].retain(|x| *x >= x_min && *x <= x_max);
        }
        // println!("coords after t = {:?}", self.coords);
    }

    fn find_index(&self, p : Pt) -> Index
    {
        let mut x = Index([0,0,0]);
        for i in 0..3 {
            x.0[i] = self.coords[i].binary_search(&p[i]).unwrap();
        }
        return x;
    }

    fn make_point(&self, idx :Index) -> Pt {
        [ self.coords[0][idx.0[0]]
        , self.coords[1][idx.0[1]]
        , self.coords[2][idx.0[2]] ]
    }

    pub fn run(&self) -> Vec<Pt> {
        const INF : u32 = 1e9 as u32;

        let mut queue = PriorityQueue::new();
        let mut dist : HashMap<Index, u32> = HashMap::new();
        let mut pred : HashMap<Index, Index> = HashMap::new();

        let start = self.source.closest_point(&self.target);

        // println!("self.source = {:?} target = {:?} start = {:?}", self.source, self.target, start);
        // println!("distance {:?}", self.source.distance(&self.target));
        // println!("coords = {:?}", self.coords);

        let s = self.find_index(start);
        dist.insert(s, 0);
        queue.push(0, s);
        
        let mut t = Index([0,0,0]);
        loop {
            match queue.pop() {
                None => break,
                Some(u) => {
                    let u_pt = self.make_point(u);
                    if self.target.distance_point(&u_pt) == 0 {
                        t = u;
                        break;
                    }
                    for v in self.neighbors(&u) {
                        let v_pt = self.make_point(v);

                        if self.obstacle_index.hits(Rect::build(u_pt, v_pt)) {
                            continue;
                        }

                        let w = if self.source.distance_point(&v_pt) == 0 {
                            0
                        } else {
                            manhatan(u_pt, v_pt)
                        };

                        let a_star_heuristic = (self.target.distance_point(&v_pt) as f64 * 1.01) as u32;

                        let old_w = *dist.get(&v).unwrap_or(&INF);
                        let new_w = dist.get(&u).unwrap_or(&INF) + w;

                        if old_w > new_w {
                            dist.insert(v, new_w);
                            pred.insert(v, u);
                            queue.push(new_w+a_star_heuristic, v);
                        }
                    }
                }
            }
        }

        let mut path = Vec::new();

        loop {
            let pt = self.make_point(t);
            path.push(pt);
            if self.source.distance_point(&pt) == 0 {
                break;
            }
            t = pred[&t];
        }

        return path;
    }

    fn neighbors(&self, idx : &Index) -> Vec<Index> {
        let mut v = Vec::new();

        let Index([x,y,z]) = *idx;

        if x > 0 {
            v.push(Index([x-1, y, z]));
        }
        if x < self.coords[0].len()-1 {
            v.push(Index([x+1, y, z]));
        }
        if y > 0 {
            v.push(Index([x, y-1, z]));
        }
        if y < self.coords[1].len()-1 {
            v.push(Index([x, y+1, z]));
        }
        if z > 0 {
            v.push(Index([x, y, z-1]));
        }
        if z < self.coords[2].len()-1 {
            v.push(Index([x, y, z+1]));
        }
        return v;
    }


}


pub fn astar(
    u : Rect,
    v : Rect,
    obstacle_index : &RTree<usize>,
    boundary: Rect) -> Route
{
    let path = AStar::new(obstacle_index, u, v, boundary).run();
    return Route {
        length : u.distance(&v),
        path : path
    };
}