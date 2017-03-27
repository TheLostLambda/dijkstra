//! Provides the vertex struct for storing vertex IDs and distance data

// Imports
use super::*;
use std::cmp::Ordering;

/// The Vertex struct represents a single node on a graph.
///
/// The id field stores a unique String identifier that can be used to reference
/// this Vertex within a graph. The dist tuple stores data for use in the Dijkstra
/// algorithm. The first element of the tuple stores the ID of a parent Vertex from
/// which the second element, the numerical distance, is calculated.
///
/// For a greater understanding of the dist field, see the shortest_path() function
/// in the base module of the dijkstra crate.
#[derive(Clone, Debug, Eq)]
pub struct Vertex {
    // Unique String identifier used to reference this vertex
    pub id: ID,
    // Temporary distance data for use in the shortest_path() function
    // Suggestion: Find a way to move this data to the shortest_path() function
    // and remove it from the Vertex struct.
    pub dist: (Option<ID>, Option<Dist>),
}

impl Vertex {
    /// Vertices can be constructed from a single String identifier.
    ///
    /// ## Example
    /// ```
    /// # #[macro_use] extern crate dijkstra;
    /// # fn main() {
    /// use dijkstra::util::*;
    /// let v = Vertex::new("A");
    /// # }
    /// ```
    ///
    /// The dist field is set to (None,None) by default.
    pub fn new(n: &str) -> Vertex {
        Vertex {
            id: n.to_string(),
            dist: (None, None),
        }
    }
}

// Implementing the Ord trait allows us to order the Vertices by their distances.
// The Ord trait allows us to sort a list of Vertices so that the low distances
// come first, the the higher distances, then finally the infinite distances (None).
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

// PartialOrd is a trait dependency of Ord and must be given a definition. Here is
// is defined in terms of the original Ord definition.
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// PartialEq is required to derive Eq (see the #[derive]) and is defined as two
// structs having the same distance as each other.
impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.dist.1 == other.dist.1
    }
}
