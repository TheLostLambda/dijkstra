pub mod util;

use std::collections::HashSet;
use self::util::*;

// Suggestion: Do away Vertex and replace with HashMap of IDs and distances

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
            None => return None,
        };

        // Set initial distance to zero with no parent node
        temp.dist = (None, Some(0));
    }
    // Create a new HashSet to track visited nodes
    let mut visited: HashSet<ID> = HashSet::new();

    // Set up the priority queue with the visited nodes removed
    let mut pq: Vec<Vertex> = g.clone()
        .verts
        .into_iter()
        .filter(|x| !visited.contains(&x.id))
        .collect();

    while !pq.is_empty() {
        if current.id == b {
            break;
        }

        visited.insert(current.clone().id);
        pq.sort();
        current = pq.remove(0);

        for (dist, terminal) in g.clone().lookup_neighbors(&current.id) {
            let mut target = g.lookup_id_mut(&terminal).unwrap();
            let t_dist = current.clone()
                .dist
                .1
                .unwrap() + dist;
            target.dist = match target {
                &mut Vertex { id: _, dist: (_, Some(dist)) } => {
                    if t_dist < dist {
                        (Some(current.clone().id), Some(t_dist))
                    } else {
                        target.dist.clone()
                    }
                }
                &mut Vertex { id: _, dist: (_, None) } => (Some(current.clone().id), Some(t_dist)),
            };
        }
        pq = g.clone()
            .verts
            .into_iter()
            .filter(|x| !visited.contains(&x.id))
            .collect();
    }

    Some(route(&g, b))
}

fn route(g: &Graph, id: &str) -> Path {
    let mut path = Vec::new();
    let mut id = id;
    // Suggestion: Explore the possibility of replacing this loop with a recursive subroutine
    loop {
        let current = match g.verts.iter().find(|x| x.id == id) {
            Some(vert) => vert,
            None => return Path { verts: Vec::new() },
        };
        path.insert(0, current.clone());
        match current.dist.0 {
            Some(ref child) => id = child,
            None => break,
        }
    }
    Path { verts: path }
}
