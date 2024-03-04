use std::path::PathBuf;

use ppmitzador::{ImagePBM, PpmFormat};

use crate::{traversal::DFSError, Graph, Node};

impl<T> Graph<T> {
    fn is_tree(&self) -> bool where T: PartialEq {
        let Some(v) = self.nodes.get(0) else { return true; };

        let mut w = vec![v];   
        let mut visited = vec![(v, 0)]; // tuples of (vertex, id) where the id is one more than the
                                        // id of its parents. if a vertex would have two ids, we have
                                        // a tree
        let mut stack = vec![v];

        let mut i = 1;
        while let Some(x) = stack.pop() {
            for y in &self.nodes {
                if self 
                    .has_adjacency(x.name, y.name)
                    .ok_or(DFSError::VertexNotFound).unwrap() && !w.contains(&y)
                {
                    w.push(y);
                    visited.push((y, i));
                    stack.push(y);
                }
            }
            i += 1;
        }

        false
    }
}
