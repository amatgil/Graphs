use graphs::{Graph, Node};


#[test]
fn dfs_basic() {
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

    let res = g.dfs(&a).unwrap();
    assert_eq!(res.0, vec![&a, &b, &c, &d]);
}

#[test]
fn dfs_advanced() {
    let a = Node::<()>::new('A', ());
    let b = Node::<()>::new('B', ());
    let c = Node::<()>::new('C', ());
    let d = Node::<()>::new('D', ());
    let e = Node::<()>::new('E', ());

    let init = vec![
        (a, vec![e, b]),
        (b, vec![a, c]),
        (c, vec![e, b]),
        (d, vec![e]),
        (e, vec![a, c, d]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();
    let res = g.dfs(&a).unwrap();

    assert_eq!(res.0, vec![&a, &e, &b, &c, &d]);
    
    let disconnected_l = vec![
        (a, vec![]),
        (b, vec![e]),
        (c, vec![d, e]),
        (d, vec![c]),
        (e, vec![b, c]),
    ];

    let disconnected = Graph::<()>::from_list(disconnected_l).unwrap();
    let res = disconnected.dfs(&a).unwrap();

    assert!(res.0.len() == 1);

    let res = disconnected.dfs(&b).unwrap();
    assert_eq!(res.0, vec![&b, &e, &c, &d]);

}
#[test]
fn trees() {
    let a = Node::<()>::new('A', ());
    let b = Node::<()>::new('B', ());
    let c = Node::<()>::new('C', ());
    let d = Node::<()>::new('D', ());
    let e = Node::<()>::new('E', ());
    let f = Node::<()>::new('F', ());
    let g = Node::<()>::new('G', ());

    let first_l = vec![
        (a, vec![e, b]),
        (b, vec![a]),
        (c, vec![e]),
        (d, vec![e]),
        (e, vec![a, c, d]),
    ];

   let second_l = vec![
        (a, vec![b,c]),
        (b, vec![d,a,e]),
        (c, vec![a,f,g]),
        (d, vec![b]),
        (e, vec![b]),
        (f, vec![c]),
        (g, vec![c]),
    ];


    let third_l = vec![
        (a, vec![e, b]),
        (b, vec![a, c]),
        (c, vec![e, b]),
        (d, vec![e]),
        (e, vec![a, c, d]),
    ];

    let complete_l = vec![
        (a, vec![  b,c,d]),
        (b, vec![a,  c,d]),
        (c, vec![a,b,  d]),
        (d, vec![a,b,c  ]),
    ];

    let bowtie_l = vec![
        (a, vec![b,c]),
        (b, vec![a,c]),
        (c, vec![a,b,e,d]),
        (d, vec![e,c]),
        (e, vec![d,c])
    ];

    let first    = Graph::from_list(first_l).unwrap();
    let second   = Graph::from_list(second_l).unwrap();
    let third    = Graph::from_list(third_l).unwrap();
    let complete = Graph::from_list(complete_l).unwrap();
    let bowtie   = Graph::from_list(bowtie_l).unwrap();

    assert!(first.is_tree().0);
    assert!(second.is_tree().0);
    assert!(!third.is_tree().0);
    assert!(!complete.is_tree().0);
    assert!(!bowtie.is_tree().0);
}

#[test]
fn bfs_basic() {
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

    let res = g.bfs(&a).unwrap();
    assert_eq!(res.0, vec![(&a, 0), (&b, 1), (&c, 1), (&d, 2)]);
    assert_eq!(res.2, 2);
}

#[test]
fn bfs_advanced() {
    let a = Node::<()>::new('A', ());
    let b = Node::<()>::new('B', ());
    let c = Node::<()>::new('C', ());
    let d = Node::<()>::new('D', ());
    let e = Node::<()>::new('E', ());

    let init = vec![
        (c, vec![d, e]),
        (b, vec![a, e]),
        (d, vec![c]),
        (e, vec![b, c]),
        (a, vec![b]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();
    let res = g.bfs(&c).unwrap();
    dbg!(&res);

    assert_eq!(res.0, vec![(&c, 0), (&d, 1), (&e, 1), (&b, 2), (&a, 3)]);
    assert_eq!(res.2, 3);
    
    let disconnected_l = vec![
        (a, vec![]),
        (b, vec![e]),
        (c, vec![d, e]),
        (d, vec![c]),
        (e, vec![b, c]),
    ];

    let disconnected = Graph::<()>::from_list(disconnected_l).unwrap();
    let res = disconnected.dfs(&a).unwrap();

    assert!(res.0.len() == 1);

    let res = disconnected.dfs(&b).unwrap();
    assert_eq!(res.0, vec![&b, &e, &c, &d]);
}

#[test]
fn bfs_advanced2() {
    let a = Node::<()>::new('A', ());
    let b = Node::<()>::new('B', ());
    let c = Node::<()>::new('C', ());
    let d = Node::<()>::new('D', ());
    let e = Node::<()>::new('E', ());

    let init = vec![
        (a, vec![e, b]),
        (b, vec![a, c]),
        (c, vec![e, b]),
        (d, vec![e]),
        (e, vec![a, c, d]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();
    let res = g.bfs(&a).unwrap();
    dbg!(&res);

    assert_eq!(res.0, vec![(&a, 0), (&e, 1), (&b, 1), (&c, 2), (&d, 2)]);
    assert_eq!(res.2, 2);
    
}
