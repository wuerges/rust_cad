use crate::rtree::*;
use crate::simplesvg::*;
use crate::geometry::Rect;

fn choose_color(id : usize) -> ColorAttr {
    let v = vec![ Color(0, 255, 0), Color(0, 0, 255), Color(255, 0, 0)
                , Color(0, 0, 0)
                , Color(0, 255, 255), Color(255, 0, 255), Color(255, 255, 0)];
    return v[id % v.len()];
}


fn make_rect(r : &Rect) -> Fig {
    return Fig::Rect(r.p1[0] as f32 / 5.0, r.p1[1] as f32 / 5.0
                    , (r.p2[0] - r.p1[0]) as f32 / 5.0
                    , (r.p2[1] - r.p1[1]) as f32 / 5.0);
}

fn make_style(id: usize) -> Attr {
    Attr::default()
    .stroke(choose_color(id))
    .stroke_width(5.0)
    .opacity(0.5)
}

fn from_rtree_rec<T>(tree : & RTree<T>, level : usize) -> Fig
{
    match tree {
        RTree::Sent => Fig::Text(0.0, 0.0, String::from("sentinel")),
        RTree::Leaf (bb, _) => make_rect(&bb).styled(make_style(level)),
        RTree::Child(bb, child) => {
            let mut v : Vec<Fig> = child.iter().map( |c| from_rtree_rec(c, level+1)).collect();
            v.push(make_rect(&bb).styled(make_style(level)));
            return Fig::Multiple(v);
        }
    }    
}

pub fn from_rtree<T>(tree : &RTree<T>) -> Svg {
    return Svg(vec![from_rtree_rec(tree, 0)], 1080, 720);
}