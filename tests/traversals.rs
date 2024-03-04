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
    dbg!(&g);
    dbg!(res);
    panic!()
}

#[test]
fn trees() {
}
