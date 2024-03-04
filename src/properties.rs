use std::{fmt::Debug, path::PathBuf}; // TODO: Remove this when done

use crate::{traversal::DFSError, Graph, Node};

impl<T> Graph<T> {
    pub fn is_tree(&self) -> bool where T: PartialEq + Debug {
        let Some(v) = self.nodes.get(0) else { return true; };

        let mut w = vec![v];   
        let mut visited = vec![(v, None)]; // tuples of (vertex, Option<Parent>)
        let mut stack = vec![v];

        while let Some(x) = stack.pop() {
            for y in &self.nodes {
                if self 
                    .has_adjacency(x.name, y.name)
                    .ok_or(DFSError::VertexNotFound).unwrap() && !w.contains(&y)
                {
                    w.push(y);
                    dbg!(&visited);
                    if visited.iter().find(|(n, parent)| n == &y && match parent {
                        Some(p) => p != &x,
                        None => true, // Finish short-circuiting, we already know this isn't a tree
                    }).is_some() { return false; }
                    println!("Didn't return, yet!");

                    visited.push((y, Some(x)));
                    stack.push(y);
                }
            }
        }

        true
    }
}
