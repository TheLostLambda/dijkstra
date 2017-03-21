use super::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq)]
pub struct Vertex {
    pub id: ID,
    pub dist: (Option<ID>,Option<Dist>),
}

impl Vertex {
    pub fn new(n: &str) -> Vertex {
        Vertex { id: n.to_string(), dist: (None,None) }
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Vertex) -> Ordering {
        if self.dist.1.is_none() {
            return Ordering::Greater;
        }
        if other.dist.1.is_none() {
            return Ordering::Less;
        }
        self.dist.1.cmp(&other.dist.1)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.dist.1 == other.dist.1
    }
}
