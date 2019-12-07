use crate::geometry::*;
use crate::rtree::*;

pub struct Route { 
    pub length : u32, 
    pub path : Vec<Pt> 
}

type Index = [u32; 3];
type Coords = [Vec<u32>; 3];

pub struct AStar<'a> {
    obstacle_index : &'a  RTree<u32>,
    boundary : Rect,
    source : Rect,
    target : Rect,
    coords : Coords
}

impl<'a> AStar<'a> {

    pub fn new(obstacle_index : &'a RTree<u32>, s :Rect, t:Rect, bound : Rect ) -> Self {
        let mut a = AStar {
            obstacle_index : obstacle_index,
            boundary : bound,
            source : s,
            target : t,
            coords : Coords::default(),
        };
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
        for i in 0..3 {
            self.coords[i].sort_unstable();
            self.coords[i].dedup();
            let x_min = self.boundary.p1[i];
            let x_max = self.boundary.p2[i];
            self.coords[i].retain(|x| *x >= x_min && *x <= x_max);
        }
    }


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