

struct Edge (usize, usize);

struct Graph<V> {
    vertices : Vec<V>,
    edges    : Vec<Edge>
}

impl<V> Graph<V> {

    pub fn add_vertex(&mut self, v : V) -> usize {
        self.vertices.push(v);
        return self.vertices.len()-1;
    }

    pub fn add_edge(&mut self, u : usize, v : usize) -> usize {
        self.edges.push(Edge(u, v));
        return self.edges.len()-1;
    }
}