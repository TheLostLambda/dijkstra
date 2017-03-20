use super::*;
use std::fmt;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Graph {
    pub verts: Vec<Vertex>,
    pub edges: Vec<Edge>
}

impl Graph {
    pub fn new(data: Vec<(Dist, &str, &str)>) -> Graph {
        let vs: HashSet<&str> = data.iter().flat_map(|&(_, a, b)| vec![a, b].into_iter()).collect();
        let vs = vs.iter().map(|x| Vertex::new(x)).collect();
        let es = data.iter().map(|&(w, a, b)| Edge { link: ( a.to_string(), b.to_string()), weight: w }).collect();
        Graph { verts: vs, edges: es }
    }

    pub fn lookup_id_mut(&mut self, id: &str) -> Option<&mut Vertex> {
        self.verts.iter_mut().find(|x| x.id == id)
    }

    pub fn lookup_edges(self, id: &str) -> Vec<(Edge,ID)> {
        self.edges.clone().into_iter().filter(|x| x.link.0 == id || x.link.1 == id)
            .map(|x| if x.link.0 == id { (x.clone(), x.clone().link.1) } else { (x.clone(), x.clone().link.0) }).collect()
    }

    pub fn route(&self, id: &str) -> Path {
        let mut path = Vec::new();
        let mut id = id;
        loop {
            let current = match self.verts.iter().find(|x| x.id == id) {
                Some(vert) => vert,
                None => return Path { verts: Vec::new() }
            };
            path.insert(0, current.clone());
            match current.dist.0 {
                Some(ref child) => id = child,
                None => break
            }
        }
        Path { verts: path }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Edges:"));
        for &Edge { link: (ref a, ref b), weight: w} in self.edges.iter() {
            try!(write!(f, "\n\t{}, {} : {}", a, b, w));
        }
        Ok(())
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt::Display::fmt(self,f));
        try!(write!(f, "\nVertices:"));
        for &Vertex { id: ref i, dist: (ref p, ref d)} in self.verts.iter() {
            let parent = match p {
                &Some(ref x) => x,
                &None => "NULL"
            };
            let dist = match d {
                &Some(x) => x.to_string(),
                &None => "INF".to_string()
            };
            try!(write!(f, "\n\t{} - {} via {}", i, dist, parent));
        }
        Ok(())
    }
}
