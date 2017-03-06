use super::*;

pub struct Vertex {
    pub id: ID,
    pub dist: (Option<ID>,Option<Dist>),
}

impl Vertex {
    pub fn new(n: &str) -> Vertex {
        Vertex { id: n.to_string(), dist: (None,None) }
    }
}
