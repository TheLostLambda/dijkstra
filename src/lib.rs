pub mod util;

use std::collections::HashSet;
use self::util::*;

// This function runs Dijkstra's shortest path algorithm on the given graph and finds the shortest path from a to b
pub fn shortest_path(a: &str, b: &str, g: &Graph) -> Option<Path> {

    //Create a mutable copy of the graph for local use
    let mut g = g.clone();

    // Declare the current node for later assignment
    let mut current = Vertex::new("");

    {
        // Set current vertex to 'a'
        let mut temp = g.lookup_id_mut(a);
        let temp = match temp {
            Some(ref mut vert) => vert,
            None => return None
        };

        // Set initial distance to zero with no parent node
        temp.dist = (None, Some(0));
    }
    // Create a new HashSet to track visited nodes
    let mut visited: HashSet<ID> = HashSet::new();

    // Set up the priority queue with the visited nodes removed
    let mut pq: Vec<Vertex> = g.clone().verts.into_iter().filter(|x| !visited.contains(&x.id)).collect();

    while !pq.is_empty() {
        if current.id == b { break; }
        
        visited.insert(current.clone().id);
        pq.sort();
        current = pq.remove(0);

        for (edge, terminal) in g.clone().lookup_edges(&current.id) {
            let mut target = g.lookup_id_mut(&terminal).unwrap();
            let t_dist = current.clone().dist.1.unwrap() + edge.weight;
            target.dist = match target {
                &mut Vertex { id: _, dist: (_,Some(dist)) } => if t_dist < dist { (Some(current.clone().id), Some(t_dist)) } else { target.dist.clone() },
                &mut Vertex { id: _, dist: (_,None) } => (Some(current.clone().id), Some(t_dist)),
            };
        }
        pq = g.clone().verts.into_iter().filter(|x| !visited.contains(&x.id)).collect();
    }

    Some(g.route(b))
}
