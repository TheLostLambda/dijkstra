use super::*;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path {
    pub verts: Vec<Vertex>,
}

impl Path {
    pub fn length(&self) -> Dist {
        if self.verts.is_empty() { 0 }
        else { match self.verts.last().unwrap().dist.1 { Some(x) => x, None => 0 } }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "[ "));
        for vert in self.verts.iter() {
            try!(write!(f, "{} ", vert.id));
        }
        try!(write!(f, "]"));
        Ok(())
    }
}
