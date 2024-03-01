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
        [true,  false, false, true],  // B
        [true,  false, false, true],  // C
        [false, true,  true,  false], // D
    ];
    let mut g = Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], [(); 4], init).unwrap();

    assert_eq!(g.get_node('C').unwrap(), &Node::new('C', ()));
    assert!(g.change_node_name('C', 'Z').is_ok());
    assert!(g.get_node('C').is_none());
    assert!(g.change_node_name('C', 'Z').is_err());
}

#[test]
fn no_dupes_allowed() {
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
        [true,  false, false, true],  // B
        [true,  false, false, true],  // C
        [false, true,  true,  false], // D
    ];
    let mut g = Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], [(); 4], init).unwrap();

    assert_eq!(g.get_node('C').unwrap(), &Node::new('C', ()));

    assert!(g.change_node_name('C', 'A').is_err()); // Ja esta agafat
    assert!(g.get_node('C').is_some());
}
