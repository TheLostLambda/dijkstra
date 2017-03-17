use super::*;
use std::fmt;

pub struct Graph {
    pub verts: Vec<Vertex>,
    pub edges: Vec<Edge>
}

impl Graph {
    pub fn new(data: Vec<(f64, &str, &str)>) -> Graph {
        let vs = data.iter().flat_map(|&(_, a, b)| vec![Vertex::new(a), Vertex::new(b)].into_iter()).collect();
        let es = data.iter().map(|&(w, a, b)| Edge { link: ( a.to_string(), b.to_string()), weight: w }).collect();
        Graph { verts: vs, edges: es }
    }

    pub fn lookup_id(&self, id: &str) -> Option<&Vertex> {
        self.verts.iter().find(|x| x.id == id)
    }

    pub fn lookup_id_mut(&mut self, id: &str) -> Option<&mut Vertex> {
        self.verts.iter_mut().find(|x| x.id == id)
    }

    pub fn lookup_edges(&self, id: &str) -> Vec<(&Edge,&ID)> {
        self.edges.iter().filter(|x| x.link.0 == id || x.link.1 == id)
            .map(|x| if x.link.0 == id { (x, &x.link.1) } else { (x, &x.link.0) }).collect()
    }
}

impl<'a> fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &Edge { link: (ref a, ref b), weight: w} in self.edges.iter() {
            try!(write!(f, "\n\t{}, {} : {}", a, b, w));
        }
        Ok(())
    }
}
