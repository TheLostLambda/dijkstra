extern crate dijkstra;
use dijkstra::util::*;

/*
  Todo:
    * Comment code
    * Write tests for the shortest_path function
    * Condense and refine code (making sure that all tests are still being passed)
    * Profile code for performance and optimize for memory usage and speed
    * Test with larger, more complicated graphs
*/

fn main() {
    let g = Graph::new(vec![(6,"A","C"), (2,"A","B"), (1,"C","B"), (3,"C","D"), (5,"B","D")]);
    println!("\nHere is the current graph: \n{}", g);

    let p = dijkstra::shortest_path("A", "D", &g).unwrap();
    println!("\nShortest path from A to D: {}", p);
    println!("Path length is: {}", p.length());

    let p = dijkstra::shortest_path("D", "B", &g).unwrap();
    println!("\nShortest path from D to B: {}", p);
    println!("Path length is: {}", p.length());

    let p = dijkstra::shortest_path("A", "C", &g).unwrap();
    println!("\nShortest path from A to C: {}", p);
    println!("Path length is: {}", p.length());
}
