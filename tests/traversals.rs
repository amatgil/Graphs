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

    assert!(first.is_tree());
    assert!(second.is_tree());
    assert!(!third.is_tree());
    assert!(!complete.is_tree());
    assert!(!bowtie.is_tree());
}
