extern crate dijkstra;
use dijkstra::util::*;

fn shortest_path_test(a: &str, b: &str) -> (String, Dist) {
    let master = Graph::new(vec![(1, "A", "L"), (6, "A", "C"), (3, "C", "D")
                                ,(5, "B", "D"), (1, "C", "B"), (8, "J", "B")
                                ,(5, "L", "J"), (3, "J", "M"), (2, "J", "K")
                                ,(1, "M", "N"), (4, "P", "N"), (2, "N", "O")
                                ,(3, "O", "I"), (2, "O", "T"), (7, "I", "H")
                                ,(2, "H", "G"), (4, "H", "F"), (1, "F", "E")
                                ,(3, "E", "G"), (2, "E", "D"), (5, "D", "Q")
                                ,(5, "G", "R"), (4, "T", "R"), (1, "R", "S")
                                ,(7, "Q", "R"), (2, "Q", "S"), (2, "A", "B")
                                ,(1, "W", "V"), (3, "V", "U"), (4, "W", "U")]);
    match dijkstra::shortest_path(a, b, &master) {
        Some(p) => (p.to_string(), p.length()),
        None => (String::new(), 0),
    }
}

#[test]
fn route_vv() {
    assert_eq!(shortest_path_test("V", "V"), ("[ V ]".to_string(), 0))
}

#[test]
fn route_wu() {
    assert_eq!(shortest_path_test("W", "U"), ("[ W U ]".to_string(), 4))
}

#[test]
fn route_db() {
    assert_eq!(shortest_path_test("D", "B"), ("[ D C B ]".to_string(), 4))
}

#[test]
fn route_ac() {
    assert_eq!(shortest_path_test("A", "C"), ("[ A B C ]".to_string(), 3))
}

#[test]
fn route_ad() {
    assert_eq!(shortest_path_test("A", "D"), ("[ A B C D ]".to_string(), 6))
}

#[test]
fn route_cr() {
    assert_eq!(shortest_path_test("C", "R"), ("[ C D Q S R ]".to_string(), 11))
}

#[test]
fn route_co() {
    assert_eq!(shortest_path_test("C", "O"), ("[ C B J M N O ]".to_string(), 15))
}

#[test]
fn route_kr() {
    assert_eq!(shortest_path_test("K", "R"), ("[ K J M N O T R ]".to_string(), 14))
}

#[test]
fn route_lw() {
    assert_eq!(shortest_path_test("L", "W"), ("".to_string(), 0))
}

#[test]
fn route_az() {
    assert_eq!(shortest_path_test("A", "Z"), ("".to_string(), 0))
}

#[test]
fn route_xd() {
    assert_eq!(shortest_path_test("X", "D"), ("".to_string(), 0))
}
