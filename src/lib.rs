pub mod util;

use std::collections::HashSet;
use self::util::*;

// This function runs Dijkstra's shortest path algorithm on the given graph and finds the shortest path from a to b
pub fn shortest_path<'a>(a: &str, b: &str, g: &'a mut Graph) -> Option<&'a mut Graph>/*Option<Path>*/ {

    {
        // Set current vertex to 'a'
        let mut temp = g.lookup_id_mut(a);
        let current: &mut Vertex = match temp {
            Some(ref mut vert) => vert,
            None => return None
        };

        // Set initial distance to zero with no parent node
        current.dist = (None, Some(0));
    }
    // Create a new HashSet to track visited nodes
    let mut visited: HashSet<ID> = HashSet::new();

    // Set current vertex
    let current = g.clone().lookup_id(a).unwrap();

    // Mark inital node as visited
    visited.insert(a.to_string());

    loop {
        for (edge, terminal) in g.clone().lookup_edges(&current.id) {
            let mut target = g.lookup_id_mut(&terminal).unwrap();
            let t_dist = current.clone().dist.1.unwrap() + edge.weight;
            target.dist = match target {
                &mut Vertex { id: _, dist: (_,Some(dist)) } => if t_dist < dist { (Some(current.clone().id), Some(dist)) } else { target.dist.clone() },
                &mut Vertex { id: _, dist: (_,None) } => (Some(current.clone().id), Some(t_dist)),
            };
        }
        break;
    }

    Some(g)

    //unimplemented!();
}
