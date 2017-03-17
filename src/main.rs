extern crate dijkstra;
use dijkstra::util::*;

fn main() {
    let mut g = Graph::new(vec![ (6.0,"A","C")
                               , (2.0,"A","B")
                               , (1.0,"C","B")
                               , (3.0,"C","D")
                               , (5.0,"B","D") ]);
    println!("Here is the current graph: {}", g);
    //println!("Result of running shortest_path(g): {}", dijkstra::shortest_path("A", "D", &mut g));
}
