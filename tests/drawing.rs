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
const I: Node<()> = Node::<()>::new('I', ());
const J: Node<()> = Node::<()>::new('J', ());
const K: Node<()> = Node::<()>::new('K', ());
const L: Node<()> = Node::<()>::new('L', ());
const M: Node<()> = Node::<()>::new('M', ());
const N: Node<()> = Node::<()>::new('N', ());
const O: Node<()> = Node::<()>::new('O', ());

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
fn drawing_tree_gerard() {
    let init = vec![
        (A, vec![B, K]),
        (B, vec![A, C, D]),
        (C, vec![B]),
        (D, vec![B, F, E, G, H]),
        (E, vec![D]),
        (F, vec![D, I, J]),
        (G, vec![D]),
        (H, vec![D]),
        (I, vec![F]),
        (J, vec![F]),
        (K, vec![A, L]),
        (L, vec![K, M, N]),
        (M, vec![L]),
        (N, vec![L]),
    ];

    let g = Graph::from_list(init).unwrap();

    g.draw_and_save_to_file("tree_gerard.ppm", 1000, 1000).unwrap();
}
