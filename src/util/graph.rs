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
        self.verts.iter().filter(|x| x.id == id).next()
    }

    pub fn lookup_id_mut(&mut self, id: &str) -> Option<&mut Vertex> {
        self.verts.iter_mut().filter(|x| x.id == id).next()
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
