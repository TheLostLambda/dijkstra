extern crate dijkstra;
use dijkstra::util::*;

fn main() {
    let mut g = Graph::new(vec![ (6,"A","C")
                               , (2,"A","B")
                               , (1,"C","B")
                               , (3,"C","D")
                               , (5,"B","D") ]);
    println!("Here is the current graph: {}", g);
    println!("Result of running shortest_path(g): \n{}", dijkstra::shortest_path("A", "D", &mut g).unwrap());
}
