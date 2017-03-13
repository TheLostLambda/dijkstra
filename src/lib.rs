pub mod util;

use std::collections::HashSet;
use self::util::*;

// This function runs Dijkstra's shortest path algorithm on the given graph and finds the shortest path from a to b
pub fn shortest_path(a: &str, b: &str, g: &mut Graph) -> Option<Path> {

    // Set current vertex to 'a'
    let mut temp = g.lookup_id_mut(a);
    let current: &mut Vertex = match temp {
        Some(ref mut vert) => vert,
        None => return None
    };

    // Set initial distance to zero with no parent node
    current.dist = (None, Some(0));

    // Create a new HashSet to track visited nodes
    let mut visited: HashSet<&Vertex> = HashSet::new();

    // Mark inital node as visited
    visited.insert(&*current);

    unimplemented!();
}
