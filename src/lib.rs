//! The dijkstra crate provides an implementation of dijkstra's shortest path algorithm and exposes
//! various types for working with it.

// Declare the util module
// This is the module in which all of the helper types are defined
pub mod util;

// Imports
use std::collections::HashSet;
use self::util::*;

/// Given a weighted graph, return the shortest path from a to b
///
/// ## Example
/// ```
/// # #[macro_use] extern crate dijkstra;
/// # fn main() {
/// use dijkstra::util::*;
/// use std::fmt::Display;
///
/// let g = Graph::new(vec![(6,"A","C"), (2,"A","B"), (1,"C","B"), (3,"C","D"), (5,"B","D")]);
/// let p = dijkstra::shortest_path("A","D",&g).unwrap();
///
/// assert_eq!(p.to_string(), "[ A B C D ]");
/// # }
/// ```
///
/// In the case that either vertex is not present in the graph, or there is no path from a to b,
/// this function will return ```None```.
pub fn shortest_path(a: &str, b: &str, g: &Graph) -> Option<Path> {

    //Create a mutable copy of the graph for local use
    let mut g = g.clone();

    // Declare the current vertex for later assignment
    let mut current = Vertex::new("");

    // This code block is required so that the compiler can be certain that the mutable reference
    // to g, the temp variable, will never interfere with later mutable references. Because of this
    // explicit block, temp falls out of scope on line 50 and the compiler can guarantee memory safety.
    {
        // Lookup the initial vertex a and store it temporarily
        let mut temp = g.lookup_id_mut(a);

        // If the lookup of a succeeded, unwrap it. If it failed, return None
        let temp = match temp {
            Some(ref mut vert) => vert,
            None => return None,
        };

        // Set initial distance to zero with no parent vertex
        temp.dist = (None, Some(0));
    }

    // Create a new HashSet to keep track of visited vertices
    let mut visited: HashSet<ID> = HashSet::new();

    loop {

        // If the current vertex is the same as our destination, return the path from a to b
        if current.id == b {
            return Some(route(&g, b));
        }

        // Set the current vertex as visited
        visited.insert(current.clone().id);

        // Set up the priority queue with the visited and infinite distance vertices removed
        let mut pq: Vec<Vertex> = g.clone()
            .verts
            .into_iter()
            .filter(|x| !visited.contains(&x.id) && !x.dist.1.is_none())
            .collect();

        // If the priority queue is empty (all vertices have been visited), return None
        if pq.is_empty() {
            return None;
        }

        // Sort the priority queue so that the closest vertices come first
        pq.sort();

        // Get the first item from the priority queue and set it as the new current vertex
        current = pq.remove(0);

        // Loop through all of the neighbors of the current vertex and update their distances
        for (dist, terminal) in g.clone().lookup_neighbors(&current.id) {

            // Set the target vertex to the vertex returned by lookup_neighbors()
            let mut target = g.lookup_id_mut(&terminal).unwrap();

            // Find the distance to the target vertex through the current vertex
            let t_dist = current.clone()
                .dist
                .1
                .unwrap() + dist;

            // Create a new vertex from target but with the distance updated to run through current
            let updated_target = Vertex { dist: (Some(current.clone().id), Some(t_dist)), ..target.clone() };

            // If the distance through the current vertex is less than the distance of the original
            // target, set target to the updated vertex. Otherwise, return the original target vertex.
            *target = if &updated_target < target {
                updated_target.clone()
            } else {
                target.clone()
            };
        }
    }
}

// This function takes a graph with defined distances and a terminal vertex then uses each vertex's
// parent node to rebuild the path to the given id.
fn route(g: &Graph, id: &str) -> Path {

    // Declare the path as a new vector
    let mut path = Vec::new();

    // Set the current id to the terminal id
    let mut current_id = id;

    // Suggestion: Explore the possibility of replacing this loop with a recursive subroutine
    // This loop adds vertices to the path until it reaches a vertex without another parent
    loop {

        // Lookup the current vertex from the current id. If it doesn't exist, return the path early.
        let current = match g.verts.iter().find(|&x| x.id == current_id) {
            Some(vert) => vert,
            None => break,
        };

        // Insert the current vertex at the beginning of the path
        path.insert(0, current.clone());

        // If the current vertex has a parent, make that the new current_id and loop.
        // Otherwise, break out of the loop and return the path.
        match current.dist.0 {
            Some(ref child) => current_id = child,
            None => break,
        }
    }

    Path { verts: path }
}
