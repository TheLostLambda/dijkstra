//! Define the Edge struct that links vertices on a graph

// Import everything from the dijkstra crate
use super::*;

// Make the type clonable
#[derive(Clone, Debug)]
/// The Edge struct represents links between vertices on a graph and contains a pair of Vertex IDs and a link weight.
pub struct Edge {
    // The link field stores a tuple of the linked vertices IDs
    pub link: (ID, ID),
    // The weight field is an unsigned integer representing the distance between the two vertices
    pub weight: Dist,
}
