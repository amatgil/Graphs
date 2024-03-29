use std::collections::HashMap;
use std::hash::Hash;

use crate::{Graph, Node};

impl<T> Graph<T> {
    pub fn is_tree(&self) -> bool where T: Eq + Hash {
        let Some(v) = self.nodes.get(0) else { return true; };

        let mut w = vec![v];   
        let mut stack = vec![v];
        let mut parents: HashMap<&Node<T>, Option<&Node<T>>> = HashMap::new();

        while let Some(x) = stack.pop() {
            for y in self.adjacent_nodes(x.name).unwrap() {
                if let Some(Some(p)) = parents.get(x) {
                    if p == &y { continue; }
                }
                if w.contains(&y) { return false; }

                w.push(y);
                stack.push(y);
                parents.insert(y, Some(x));
            }
        }

        true
    }
}
