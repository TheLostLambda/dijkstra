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
    current.dist = (None, Some(0.0));

    unimplemented!();
}
