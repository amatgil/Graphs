use graphs::{Graph, Node};

// These must be checked manually

const A: Node<()> = Node::<()>::new('A', ());
const B: Node<()> = Node::<()>::new('B', ());
const C: Node<()> = Node::<()>::new('C', ());
const D: Node<()> = Node::<()>::new('D', ());
const E: Node<()> = Node::<()>::new('E', ());
const F: Node<()> = Node::<()>::new('F', ());
const G: Node<()> = Node::<()>::new('G', ());
const H: Node<()> = Node::<()>::new('H', ());

#[test]
fn drawing_tree_basic() {
    let init = vec![
        (C, vec![D, E]),
        (B, vec![A, E]),
        (D, vec![C]),
        (E, vec![B, C]),
        (A, vec![B]),
    ];

    let g = Graph::from_list(init).unwrap();

    g.draw_and_save_to_file("tree_basic.ppm", 1000, 1000).unwrap();
}

#[test]
fn drawing_tree_wide() {
    let init = vec![
        (A, vec![B, C, D, E, F]),
        (B, vec![A]),
        (C, vec![A]),
        (D, vec![A]),
        (E, vec![A]),
        (F, vec![A]),
    ];

    let g = Graph::from_list(init).unwrap();

    g.draw_and_save_to_file("tree_wide.ppm", 1000, 1000).unwrap();
}

#[test]
fn drawing_tree_tall() {
    let init = vec![
        (A, vec![B, C]),
        (B, vec![A, D, E]),
        (C, vec![A]),
        (D, vec![B, F, G]),
        (E, vec![B]),
        (F, vec![D]),
        (G, vec![D]),
    ];

    let g = Graph::from_list(init).unwrap();

    g.draw_and_save_to_file("tree_tall.ppm", 1000, 1000).unwrap();
}

#[test]
fn drawing_tree_tall2() {
    let init = vec![
        (A, vec![B, C, H]),
        (B, vec![A, D, E]),
        (C, vec![A]),
        (D, vec![B, F, G]),
        (E, vec![B]),
        (F, vec![D]),
        (G, vec![D]),
        (H, vec![A]),
    ];

    let g = Graph::from_list(init).unwrap();

    g.draw_and_save_to_file("tree_tall2.ppm", 1000, 1000).unwrap();
}
