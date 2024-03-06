// Contains helper functions that traverse the graph, like BFS and DFS

use std::{collections::VecDeque, fmt::Debug};

use crate::{Graph, Node};

#[derive(Debug, thiserror::Error)]
pub enum TraversalError {
    #[error("starting vertex was not found in the graph")]
    VertexNotFound,
}

impl<T> Graph<T> {
    /// Returns a vector of tuples that contain
    /// - The nodes in the c.c.
    /// - The edges that were explored
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

    /// Returns a vector of tuples that contain, in order:
    /// - The nodes in the c.c. with their distances
    /// - The edges that form the tree
    /// - The eccentricity (maximum distance) from the given node
    pub fn bfs<'a, 'b>(
        &'a self,
        v: &'b Node<T>,
    ) -> Result<(Vec<(&'a Node<T>, usize)>, Vec<(&'a Node<T>, &'a Node<T>)>, usize), TraversalError>
    where
        'b: 'a,
        T: PartialEq
    {
        let mut w = vec![v];
        let mut distances = vec![0];
        let mut queue = VecDeque::from([v]);
        let mut arestes = vec![];

        while let Some(x) = queue.pop_front() {
            let parent_distance = *distances.last().unwrap();
            for y in self.adjacent_nodes(x.name).ok_or(TraversalError::VertexNotFound)? {
                if !w.contains(&y)
                {
                    w.push(y);
                    distances.push(parent_distance + 1);
                    queue.push_back(y);
                    arestes.push((x, y));
                }
            }
        }
        let max_dist = *distances.last().unwrap();

        Ok((w.into_iter().zip(distances).collect(), arestes, max_dist))
    }
}
