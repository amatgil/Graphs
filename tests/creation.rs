use graphs::{Graph, Node};

#[test]
fn from_a_matrix() {
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
        [false, true, true, false], // A
        [true, false, false, true], // B
        [true, false, false, true], // C
        [false, true, true, false], // D
    ];

    let init_not_symmetric = [
        //   A      B      C      D
        [false, true, true, false], // A
        [true, false, false, true], // B
        [true, false, false, true], // C
        [true, true, true, false],  // D
    ];

    let init_not_zeros = [
        //   A      B      C      D
        [false, true, true, false], // A
        [true, true, false, true],  // B
        [true, false, false, true], // C
        [false, true, true, false], // D
    ];

    assert!(Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], init_not_symmetric).is_err());
    assert!(Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], init_not_zeros).is_err());

    let g = Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], init).unwrap();

    assert!(g.has_adjacency('A', 'B').unwrap());
    assert!(g.has_adjacency('A', 'C').unwrap());
    assert!(!g.has_adjacency('A', 'D').unwrap());

    assert!(g.has_adjacency('B', 'A').unwrap());
    assert!(g.has_adjacency('C', 'A').unwrap());
    assert!(!g.has_adjacency('D', 'A').unwrap());

    assert!(g.has_adjacency('C', 'A').unwrap());
    assert!(g.has_adjacency('C', 'D').unwrap());
    assert!(!g.has_adjacency('C', 'B').unwrap());

    assert!(g.has_adjacency('A', 'C').unwrap());
    assert!(g.has_adjacency('D', 'C').unwrap());
    assert!(!g.has_adjacency('B', 'C').unwrap());

    assert!(g.has_adjacency('A', 'F').is_none());
}

#[test]
fn from_a_list() {
    // Graph should be something like
    //┌─────A
    //│     │
    //│     │
    //│     C────┐
    //│          │
    //│          │
    //B──────────D

    let a = Node::<()>::new('A', None);
    let b = Node::<()>::new('B', None);
    let c = Node::<()>::new('C', None);
    let d = Node::<()>::new('D', None);

    let init: Vec<(Node<()>, &Vec<Node<()>>)> = vec![
        (a, &vec![b, c]),
        (b, &vec![a, d]),
        (c, &vec![a, d]),
        (d, &vec![b, c]),
    ];

    let g = Graph::<()>::from_list(&*init).unwrap();
}
