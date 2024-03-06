use graphs::{Graph, Node};


const A: Node<()> = Node::<()>::new('A', ());
const B: Node<()> = Node::<()>::new('B', ());
const C: Node<()> = Node::<()>::new('C', ());
const D: Node<()> = Node::<()>::new('D', ());
const E: Node<()> = Node::<()>::new('E', ());
const F: Node<()> = Node::<()>::new('F', ());
const G: Node<()> = Node::<()>::new('G', ());

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

    let init = vec![
        (A, vec![B, C]),
        (B, vec![A, D]),
        (C, vec![A, D]),
        (D, vec![B, C]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();

    let res = g.dfs(&A).unwrap();
    assert_eq!(res.0, vec![&A, &B, &C, &D]);
}

#[test]
fn dfs_advanced() {
    let init = vec![
        (A, vec![E, B]),
        (B, vec![A, C]),
        (C, vec![E, B]),
        (D, vec![E]),
        (E, vec![A, C, D]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();
    let res = g.dfs(&A).unwrap();

    assert_eq!(res.0, vec![&A, &E, &B, &C, &D]);
    
    let disconnected_l = vec![
        (A, vec![]),
        (B, vec![E]),
        (C, vec![D, E]),
        (D, vec![C]),
        (E, vec![B, C]),
    ];

    let disconnected = Graph::<()>::from_list(disconnected_l).unwrap();
    let res = disconnected.dfs(&A).unwrap();

    assert!(res.0.len() == 1);

    let res = disconnected.dfs(&B).unwrap();
    assert_eq!(res.0, vec![&B, &E, &C, &D]);

}
#[test]
fn trees() {
    let first_l = vec![
        (A, vec![E, B]),
        (B, vec![A]),
        (C, vec![E]),
        (D, vec![E]),
        (E, vec![A, C, D]),
    ];

   let second_l = vec![
        (A, vec![B,C]),
        (B, vec![D,A,E]),
        (C, vec![A,F,G]),
        (D, vec![B]),
        (E, vec![B]),
        (F, vec![C]),
        (G, vec![C]),
    ];


    let third_l = vec![
        (A, vec![E, B]),
        (B, vec![A, C]),
        (C, vec![E, B]),
        (D, vec![E]),
        (E, vec![A, C, D]),
    ];

    let complete_l = vec![
        (A, vec![  B,C,D]),
        (B, vec![A,  C,D]),
        (C, vec![A,B,  D]),
        (D, vec![A,B,C  ]),
    ];

    let butterfly_l = vec![
        (A, vec![B,C]),
        (B, vec![A,C]),
        (C, vec![A,B,E,D]),
        (D, vec![E,C]),
        (E, vec![D,C])
    ];

    let first         = Graph::from_list(first_l).unwrap();
    let second        = Graph::from_list(second_l).unwrap();
    let third         = Graph::from_list(third_l).unwrap();
    let complete      = Graph::from_list(complete_l).unwrap();
    let butterfly_l   = Graph::from_list(butterfly_l).unwrap();

    assert!(first.is_tree().0);
    assert!(second.is_tree().0);
    assert!(!third.is_tree().0);
    assert!(!complete.is_tree().0);
    assert!(!butterfly_l.is_tree().0);
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

    let init = vec![
        (A, vec![B, C]),
        (B, vec![A, D]),
        (C, vec![A, D]),
        (D, vec![B, C]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();

    let res = g.bfs(&A).unwrap();
    assert_eq!(res.0, vec![(&A, 0), (&B, 1), (&C, 1), (&D, 2)]);
    assert_eq!(res.2, 2);
}

#[test]
fn bfs_advanced() {
    let init = vec![
        (C, vec![D, E]),
        (B, vec![A, E]),
        (D, vec![C]),
        (E, vec![B, C]),
        (A, vec![B]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();
    let res = g.bfs(&C).unwrap();
    dbg!(&res);

    assert_eq!(res.0, vec![(&C, 0), (&D, 1), (&E, 1), (&B, 2), (&A, 3)]);
    assert_eq!(res.2, 3);
    
    let disconnected_l = vec![
        (A, vec![]),
        (B, vec![E]),
        (C, vec![D, E]),
        (D, vec![C]),
        (E, vec![B, C]),
    ];

    let disconnected = Graph::<()>::from_list(disconnected_l).unwrap();
    let res = disconnected.dfs(&A).unwrap();

    assert!(res.0.len() == 1);

    let res = disconnected.dfs(&B).unwrap();
    assert_eq!(res.0, vec![&B, &E, &C, &D]);
}

#[test]
fn bfs_advanced2() {
    let init = vec![
        (A, vec![E, B]),
        (B, vec![A, C]),
        (C, vec![E, B]),
        (D, vec![E]),
        (E, vec![A, C, D]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();
    let res = g.bfs(&A).unwrap();
    dbg!(&res);

    assert_eq!(res.0, vec![(&A, 0), (&E, 1), (&B, 1), (&C, 2), (&D, 2)]);
    assert_eq!(res.2, 2);
    
}

#[test]
fn connectedness() {
    let init = vec![
        (A, vec![B, C]),
        (B, vec![A, D]),
        (C, vec![A, D]),
        (D, vec![B, C]),
    ];

    let g = Graph::<()>::from_list(init).unwrap();

    assert!(g.is_connex());

    let disconnected_l = vec![
        (A, vec![]),
        (B, vec![E]),
        (C, vec![D, E]),
        (D, vec![C]),
        (E, vec![B, C]),
    ];

    let disconnected = Graph::<()>::from_list(disconnected_l).unwrap();
    assert!(!disconnected.is_connex())
}
