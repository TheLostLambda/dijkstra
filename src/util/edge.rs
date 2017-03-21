/// Define the edge type that links vertices on a graph

// Import everything from the dijkstra crate
use super::*;

// Make the type clonable
#[derive(Clone)]
// Define the Edge type that represents links between vertices on a graph
pub struct Edge {
    // The link field stores a tuple of the linked vertices IDs
    pub link: (ID, ID),
    // The weight field is an unsigned integer representing the distance between the two vertices
    pub weight: Dist,
}
