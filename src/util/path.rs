use super::*;

pub struct Path {
    pub verts: Vec<Vertex>,
}

impl Path {
    pub fn new() -> Path {
        Path { verts: Vec::new() }
    }
}
