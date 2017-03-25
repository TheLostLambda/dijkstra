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
    pub fn new(n: &str) -> Vertex {
        Vertex {
            id: n.to_string(),
            dist: (None, None),
        }
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
