// Contains helper functions that traverse the graph, like BFS and DFS

use std::collections::VecDeque;

use thiserror::Error;

use crate::{Graph, Node};

#[derive(Debug, Error)]
pub enum TraversalError {
    #[error("starting vertex was not found in the graph")]
    VertexNotFound,
}

impl<T> Graph<T> {
    pub fn dfs<'a, 'b>(
        &'a self,
        v: &'b Node<T>,
    ) -> Result<(Vec<&'a Node<T>>, Vec<(&'a Node<T>, &'a Node<T>)>), TraversalError>
    where
        'b: 'a,
        T: PartialEq
    {
        let mut w = vec![v];
        let mut stack = vec![v];
        let mut arestes = vec![];

        while let Some(x) = stack.pop() {
            for y in self.adjacent_nodes(x.name).ok_or(TraversalError::VertexNotFound)? {
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

    pub fn bfs<'a, 'b>(
        &'a self,
        v: &'b Node<T>,
    ) -> Result<(Vec<(&'a Node<T>, usize)>, Vec<(&'a Node<T>, &'a Node<T>)>), TraversalError>
    where
        'b: 'a,
        T: PartialEq
    {
        let mut w = vec![v];
        let mut distances = vec![0]; let mut d = 0;
        let mut queue = VecDeque::from([v]);
        let mut arestes = vec![];

        while let Some(x) = queue.pop_front() {
            for y in self.adjacent_nodes(x.name).ok_or(TraversalError::VertexNotFound)? {
                if !w.contains(&y)
                {
                    w.push(y);
                    distances.push(d + 1);
                    queue.push_back(y);
                    arestes.push((x, y));
                }
            }
            d += 1;
        }

        Ok((w.into_iter().zip(distances).collect(), arestes))
    }
}
