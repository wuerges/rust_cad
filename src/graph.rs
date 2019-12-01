

struct Edge (usize, usize);

pub struct Graph<V> {
    vertices : Vec<V>,
    edges    : Vec<Edge>
}

impl<V> Graph<V> {

    pub fn new( vs : Vec<V> ) -> Self {
        Graph {
            vertices : vs,
            edges    : Vec::new()
        }
    }

    pub fn add_vertex(&mut self, v : V) -> usize {
        self.vertices.push(v);
        return self.vertices.len()-1;
    }

    pub fn add_edge(&mut self, u : usize, v : usize) -> usize {
        self.edges.push(Edge(u, v));
        return self.edges.len()-1;
    }
}