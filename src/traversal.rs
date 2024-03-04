// Contains helper functions that traverse the graph, like BFS and DFS

use thiserror::Error;

use crate::{Graph, Node};

#[derive(Debug, Error)]
enum DFSError {
    #[error("starting vertex was not found in the graph")]
    VertexNotFound,
}

impl<T> Graph<T> {
    fn dfs<'a, 'b>(
        &'a self,
        v: &'b Node<T>,
    ) -> Result<(Vec<&Node<T>>, Vec<(&Node<T>, &Node<T>)>), DFSError>
    where
        'b: 'a,
    {
        let mut w = vec![];
        let mut stack = vec![v];
        let mut arestes = vec![];

        while let Some(x) = stack.pop() {
            for y in &self.nodes {
                if self
                    .has_adjacency(x.name, y.name)
                    .ok_or(DFSError::VertexNotFound)?
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
