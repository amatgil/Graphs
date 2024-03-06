use std::collections::HashMap;
use std::hash::Hash;

use crate::{Graph, Node};

impl<T> Graph<T> {
    /// Check if the graph is a tree (or, equivalently, if it doesn't have any cycles). Returns
    /// (true, None) if it's a tree and (false, Some(n)) if it contains a cycle with n being one of
    /// the nodes in the first cycle found
    pub fn is_tree(&self) -> (bool, Option<&Node<T>>) where T: Eq + Hash {
        let Some(v) = self.nodes.get(0) else { return (true, None); };

        let mut w = vec![v];   
        let mut stack = vec![v];
        let mut parents: HashMap<&Node<T>, Option<&Node<T>>> = HashMap::new();

        while let Some(x) = stack.pop() {
            for y in self.adjacent_nodes(x.name).unwrap() {
                if let Some(Some(p)) = parents.get(x) {
                    if p == &y { continue; }
                }
                if w.contains(&y) { return (false, Some(y)); }

                w.push(y);
                stack.push(y);
                parents.insert(y, Some(x));
            }
        }

        (true, None)
    }

    pub fn is_connex(&self) -> bool where T: Eq + Hash {
        if let Some(v) = self.nodes.get(0) {
            self.nodes.len() == self.bfs(v).expect("Can not error: vertex must be in the graph").0.len()
        }  else {
            true // empty graph is connex
        }
    }
}
