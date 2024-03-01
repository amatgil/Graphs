use graphs::{Graph, Node};

#[test]
fn get_basic() {
    // Graph should be something like
    //┌─────A     
    //│     │     
    //│     │     
    //│     C────┐
    //│          │
    //│          │
    //B──────────D

    let init = [
    //   A      B      C      D
        [false, true,  true,  false], // A
        [true,  false, false, true], // B
        [true,  false, false, true], // C
        [false, true,  true,  false], // D
    ];
    let g = Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], init).unwrap();

    assert_eq!(g.get_node('C').unwrap(), &Node::new('C', None));
}
