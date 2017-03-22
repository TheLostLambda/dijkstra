//! Provides the Path type returned by the shortest_path() function and some methods for reading it

// Imports
use super::*;
use std::fmt;

/// The Path struct stores a series of vertices.
///
/// Each of these vertices is connected to the vertices before and after it and all the vertices are
/// ordered with the first being the starting vertex and the last being the destination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path {
    // Verts stores the list of vertices that comprise the path
    pub verts: Vec<Vertex>,
}

// Suggestion: Should there be a new() method or is manually constructing the struct tolerable?
impl Path {
    /// Constructs a Path from a vector of vertices
    ///
    /// ## Example
    /// ```
    /// # #[macro_use] extern crate dijkstra;
    /// # fn main() {
    /// use dijkstra::util::*;
    /// let p = Path::new(vec![Vertex::new("A"), Vertex::new("B"), Vertex::new("C")]);
    /// # }
    /// ```
    pub fn new(data: Vec<Vertex>) -> Path {
        // Wrap the vector in the Path struct
        Path { verts: data }
    }
    /// The length method returns the total length of the path.
    ///
    /// ## Example
    /// ```
    /// # #[macro_use] extern crate dijkstra;
    /// # fn main() {
    /// use dijkstra::util::*;
    /// let (mut a, mut b, mut c) = (Vertex::new("A"), Vertex::new("B"), Vertex::new("C"));
    /// a.dist = (None, Some(0));
    /// b.dist = (Some("A".to_string()), Some(2));
    /// c.dist = (Some("B".to_string()), Some(3));
    ///
    /// let p = Path::new(vec![a, b, c]);
    /// assert_eq!(p.length(), 3);
    /// # }
    /// ```
    ///
    /// If the path is empty (it contains no vertices) this function will return a length of 0.
    pub fn length(&self) -> Dist {
        // If the path is empty, return a length of 0
        if self.verts.is_empty() { 0 }
        // Otherwise get the last vertex in the vector and return its numerical distance
        else { match self.verts.last().unwrap().dist.1 { Some(x) => x, None => 0 } }
    }
}

// This implements the Display trait which allows for the pretty printing of a path.
// In this particular case, the path will be printed as a square-bracketed list of vertex IDs.
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
