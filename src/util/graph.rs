//! Provides a weighted graph type and provides several methods for working with it

// Import everything from the dijkstra crate
use super::*;
// Import the fmt crate in order to implement the Display and Debug traits
use std::fmt;
// Import the HashSet type in order to remove duplicates objects from vectors
use std::collections::HashSet;

#[derive(Clone)]
/// The Graph struct stores a weighted graph structure.
///
/// The first vector is a list of all of the Vertices within the graph. The second stores a list of
/// all of the Edges connecting the Vertices.
pub struct Graph {
    // Verts stores all of the nodes within the graph
    pub verts: Vec<Vertex>,
    // Edges stores all of the connections within the graph
    pub edges: Vec<Edge>
}

impl Graph {
    /// Graphs are constructed from a vector of three-tuples.
    ///
    /// ## Example
    /// ```
    /// # #[macro_use] extern crate dijkstra;
    /// # fn main() {
    /// use dijkstra::util::*;
    /// let g = Graph::new(vec![(6,"A","C"), (2,"A","B"), (1,"C","B"), (3,"C","D"), (5,"B","D")]);
    /// # }
    /// ```
    ///
    /// The tuple format is as follows: ```(weight, id_a, id_b)```
    /// Keep in mind that weights must be unsigned integers and that ids must be string slices.
    pub fn new(data: Vec<(Dist, &str, &str)>) -> Graph {
        // Get an iterator over the data and for each tuple return an iterator of both IDs.
        // flat_map then flattens the iterator of iterators into a single iterator which
        // is finally collected into a HashSet. This collection into a HashSet removes duplicates.
        let vs: HashSet<&str> = data.iter().flat_map(|&(_, a, b)| vec![a, b].into_iter()).collect();
        // Map over the HashSet of IDs and create an iterator of new Vertices. Collect these into a vector.
        let vs = vs.iter().map(|x| Vertex::new(x)).collect();
        // This line maps over the original data and converts the three-tuples into Edge structs.
        // They are collected in a new vector of Edges.
        let es = data.iter().map(|&(w, a, b)| Edge { link: ( a.to_string(), b.to_string()), weight: w }).collect();
        // Return the Graph with the processed vertices and edges
        Graph { verts: vs, edges: es }
    }

    /// Lookup a Vertex in a graph given an ID and (maybe) return a mutable reference to that Vertex.
    ///
    /// ## Example
    /// ```
    /// # #[macro_use] extern crate dijkstra;
    /// # fn main() {
    /// use dijkstra::util::*;
    /// let mut g = Graph::new(vec![(6,"A","C"), (2,"A","B"), (1,"C","B"), (3,"C","D"), (5,"B","D")]);
    ///
    /// {
    ///     let mut temp = g.lookup_id_mut("A").unwrap();
    ///     temp.dist = (None, Some(0));
    /// }
    ///
    /// assert_eq!(g.lookup_id_mut("A").unwrap(), &Vertex { id: "A".to_string(), dist: (None, Some(0))});
    /// # }
    /// ```
    ///
    /// Note that this method returns an Option type. If the ID given can not be found in the graph,
    /// the function will return ```None```; otherwise, the method will return ```Some(/*Mutable Vertex Reference*/)```.
    pub fn lookup_id_mut(&mut self, id: &str) -> Option<&mut Vertex> {
        // Get an iterator of mutable references and if the desired ID is present find will return
        // Some(&mut Vertex), if the desired ID is not present, None is returned.
        self.verts.iter_mut().find(|x| x.id == id)
    }

    pub fn lookup_neighbors(self, id: &str) -> Vec<(Edge,ID)> {
        self.edges.iter().filter(|x| x.link.0 == id || x.link.1 == id)
            .map(|x| if x.link.0 == id { (x.clone(), x.clone().link.1) } else { (x.clone(), x.clone().link.0) }).collect()
    }

    pub fn route(&self, id: &str) -> Path {
        let mut path = Vec::new();
        let mut id = id;
        loop {
            let current = match self.verts.iter().find(|x| x.id == id) {
                Some(vert) => vert,
                None => return Path { verts: Vec::new() }
            };
            path.insert(0, current.clone());
            match current.dist.0 {
                Some(ref child) => id = child,
                None => break
            }
        }
        Path { verts: path }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Edges:"));
        for &Edge { link: (ref a, ref b), weight: w} in self.edges.iter() {
            try!(write!(f, "\n\t{}, {} : {}", a, b, w));
        }
        Ok(())
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt::Display::fmt(self,f));
        try!(write!(f, "\nVertices:"));
        for &Vertex { id: ref i, dist: (ref p, ref d)} in self.verts.iter() {
            let parent = match p {
                &Some(ref x) => x,
                &None => "NULL"
            };
            let dist = match d {
                &Some(x) => x.to_string(),
                &None => "INF".to_string()
            };
            try!(write!(f, "\n\t{} - {} via {}", i, dist, parent));
        }
        Ok(())
    }
}
