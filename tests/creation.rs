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

    assert!(Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], [(); 4], init_not_symmetric).is_err());
    assert!(Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], [(); 4], init_not_zeros).is_err());

    let g = Graph::<()>::from_matrix(['A', 'B', 'C', 'D'], [(); 4], init).unwrap();

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

    let a = Node::<()>::new('A', ());
    let b = Node::<()>::new('B', ());
    let c = Node::<()>::new('C', ());
    let d = Node::<()>::new('D', ());

    let init = vec![
        (a, vec![b, c]),
        (b, vec![a, d]),
        (c, vec![a, d]),
        (d, vec![b, c]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();
    dbg!(&g);

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
fn from_a_list_again() {
    // Graph should be something like (drawn with https://asciiflow.com/#/)
    //          A   <---- Lone node
    //
    //   B      C──────D
    //   │      │
    //   │      │
    //   │      │
    //   └───── E
    let a = Node::<()>::new('A', ());
    let b = Node::<()>::new('B', ());
    let c = Node::<()>::new('C', ());
    let d = Node::<()>::new('D', ());
    let e = Node::<()>::new('E', ());

    let init = vec![
        (a, vec![]),
        (b, vec![e]),
        (c, vec![d, e]),
        (d, vec![c]),
        (e, vec![b, c]),
    ];

    let init_no_sym = vec![
        (a, vec![]),
        (b, vec![e]),
        (c, vec![d]),
        (d, vec![c]),
        (e, vec![b, c]),
    ];

    let init_loop = vec![
        (a, vec![]),
        (b, vec![e, b]),
        (c, vec![d]),
        (d, vec![c]),
        (e, vec![b, c]),
    ];

    let init_repeated = vec![
        (a, vec![]),
        (a, vec![e, b]),
        (c, vec![d]),
        (d, vec![c]),
        (e, vec![b, c]),
    ];

    // e ~ c but c !~ e (oops)
    assert!(Graph::<()>::from_list(init_no_sym).is_err()); // TODO: fix later

    // b ~ b lmao
    assert!(Graph::<()>::from_list(init_loop).is_err());

    assert!(Graph::<()>::from_list(init_repeated).is_err());

    let g = Graph::<()>::from_list(init).unwrap();
    dbg!(&g);

    assert!(!g.has_adjacency('A', 'B').unwrap());
    assert!(!g.has_adjacency('A', 'C').unwrap());
    assert!(!g.has_adjacency('A', 'D').unwrap());
    assert!(!g.has_adjacency('A', 'E').unwrap());

    assert!(!g.has_adjacency('B', 'C').unwrap());
    assert!(!g.has_adjacency('B', 'D').unwrap());
    assert!(g.has_adjacency('B', 'E').unwrap());

    assert!(g.has_adjacency('C', 'D').unwrap());
    assert!(g.has_adjacency('C', 'E').unwrap());

    assert!(!g.has_adjacency('E', 'D').unwrap());

    assert!(g.has_adjacency('A', 'F').is_none());
    assert!(g.has_adjacency('Z', 'F').is_none());
}

#[test]
fn batch_adjacencies() {
    // Graph should be something like
    //┌─────A
    //│     │
    //│     │
    //│     C────┐
    //│          │
    //│          │
    //B──────────D

    let a = Node::<()>::new('A', ());
    let b = Node::<()>::new('B', ());
    let c = Node::<()>::new('C', ());
    let d = Node::<()>::new('D', ());

    let init = vec![
        (a, vec![b, c]),
        (b, vec![a, d]),
        (c, vec![a, d]),
        (d, vec![b, c]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();

    assert!(g.adjacent_nodes('A').unwrap().contains(&&b));
    assert!(g.has_adjacency('A', 'B').unwrap());
    assert!(g.adjacent_nodes('A').unwrap().contains(&&c));
    assert!(!g.adjacent_nodes('A').unwrap().contains(&&d));

    assert!(g.adjacent_nodes('B').unwrap().contains(&&a));
    assert!(g.adjacent_nodes('C').unwrap().contains(&&a));
    assert!(!g.adjacent_nodes('D').unwrap().contains(&&a));

    assert!(g.adjacent_nodes('C').unwrap().contains(&&a));
    assert!(g.adjacent_nodes('C').unwrap().contains(&&d));
    assert!(!g.adjacent_nodes('C').unwrap().contains(&&b));

    assert!(g.adjacent_nodes('A').unwrap().contains(&&c));
    assert!(g.adjacent_nodes('D').unwrap().contains(&&c));
    assert!(!g.adjacent_nodes('B').unwrap().contains(&&c));

    assert!(g.adjacent_nodes('F').is_none());
}
