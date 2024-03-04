// Contains helper functions that traverse the graph, like BFS and DFS

use thiserror::Error;

use crate::{Graph, Node};

#[derive(Debug, Error)]
pub enum DFSError {
    #[error("starting vertex was not found in the graph")]
    VertexNotFound,
}

impl<T> Graph<T> {
    pub fn dfs<'a, 'b>(
        &'a self,
        v: &'b Node<T>,
    ) -> Result<(Vec<&'a Node<T>>, Vec<(&'a Node<T>, &'a Node<T>)>), DFSError>
    where
        'b: 'a,
        T: PartialEq
    {
        let mut w = vec![v];
        let mut stack = vec![v];
        let mut arestes = vec![];

        while let Some(x) = stack.pop() {
            for y in self.adjacent_nodes(x.name).ok_or(DFSError::VertexNotFound)? {
                if !w.contains(&y)
                {
                    w.push(y);
                    stack.push(y);
                    arestes.push((x, y));
                }
            }
        }

        Ok((w, arestes))
    }
}
