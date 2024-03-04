use std::{hash::Hash, mem};
use std::
    fmt::{Debug, Display}
;

use thiserror::Error;
use utils::{coords_to_idx, has_duplicates, idx_to_coords};

use crate::utils::dedup;

mod utils;
mod properties;
mod traversal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: AdjMatrix,
}

/// The adjacency matrix: must always be symmetrical*, binary and has zeros across the diagonal. true
/// means the two nodes are adjacent, false means they're not
#[derive(Clone, Debug, PartialEq, Eq)]
struct AdjMatrix {
    values: Vec<bool>,
    n: usize,
}

impl AdjMatrix {
    pub fn from_mat<const N: usize>(m: [[bool; N]; N]) -> Result<Self, FromMatrixError> {
        // Check for symmetry:
        for y in 0..N {
            for x in 0..N {
                if m[x][y] != m[y][x] {
                    return Err(FromMatrixError::IsNotSymmetric);
                }
            }
            if m[y][y] {
                return Err(FromMatrixError::NonEmptyDiagonal);
            }
        }
        Ok(AdjMatrix {
            values: m.into_iter().flatten().collect(),
            n: N,
        })
    }

    /// Panics if either a or b are out of bounds
    fn has_adjacency(&self, a: usize, b: usize) -> Option<bool> {
        if a > self.n || b > self.n {
            None
        } else {
            // Matrix is symmetric, a and b can be swapped
            Some(self.values[coords_to_idx(a, b, self.n)])
        }
    }

    /// Assumes x is in bounds
    fn get_adjacent(&self, x: usize) -> &[bool] {
        &self.values[x*self.n..(x+1)*self.n]
    }
}

/// Basic Node type, which the graph connects. Note that changing its name or its value (if any)
/// will NOT change the structure of the graph.
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct Node<T> {
    pub name: char,
    pub value: T,
}

impl<T> Node<T> {
    /// Convenience, normal constructor may also be used without problems
    pub fn new(name: char, value: T) -> Self {
        Self { name, value }
    }
}


#[derive(Error, Debug)]
pub enum FromMatrixError {
    #[error("input matrix was not symmetrical")]
    IsNotSymmetric,
    #[error("the names of the nodes contain duplicates")]
    NodesArentUnique,
    #[error("the main diagonal containes one or more 'true's")]
    NonEmptyDiagonal,
}

#[derive(Error, Debug)]
pub enum FromListError {
    #[error("the names of the nodes contain duplicates")]
    NodesArentUnique,
    #[error("nodes may not be adjacent to themselves")]
    SelfReferentialNode,
    #[error("the lists do not agree on adjacency")]
    MalformedLists,
}

impl<T> Graph<T> {
    /// TODO: Change the bounds from Hash + Eq to something easier? Though these seem optimal
    ///
    /// This function is probably pretty awful in complexity, space and efficiency and should likely be
    /// rewritten. TODO as well
    pub fn from_list(lists: Vec<(Node<T>, Vec<Node<T>>)>) -> Result<Self, FromListError>
    where
        Node<T>: Hash + Eq,
    {
        // Pick out nodes from the indexes and lists
        // This determines the order in the matrix
        let mut nodes: Vec<&Node<T>> = Vec::new();
        for (main, adjs) in &lists {
            nodes.push(main);
            for n in adjs {
                nodes.push(n);
            }
        }

        dedup(&mut nodes);

        if has_duplicates(nodes.iter().map(|n| n.name)) {
            return Err(FromListError::NodesArentUnique);
        }

        let ordre = nodes.len();
        let mut matriu = vec![None; ordre * ordre];
        for i in 0..ordre {
            matriu[coords_to_idx(i, i, ordre)] = Some(false);
        }

        for (i, u) in nodes.iter().enumerate() {
            for (j, v) in nodes.iter().enumerate() {
                let index = coords_to_idx(i, j, ordre);
                let index_symmetric = coords_to_idx(i, j, ordre);

                let adj_list = &lists
                    .iter()
                    .find(|(x, _)| &x == u)
                    .ok_or(FromListError::MalformedLists)?
                    .1;
                if adj_list.contains(v) {
                    // u ~ v ?
                    // If we already said there hadn't, the list is not well formed
                    if matriu[index] == Some(false) || matriu[index_symmetric] == Some(false) {
                        return Err(FromListError::MalformedLists);
                    }
                    matriu[index] = Some(true);
                    matriu[index_symmetric] = Some(true);
                } else {
                    // If we already said there had, the list is not well formed
                    if matriu[index] == Some(true) || matriu[index_symmetric] == Some(true) {
                        return Err(FromListError::MalformedLists);
                    }
                    matriu[index] = Some(false);
                    matriu[index_symmetric] = Some(false);
                }
            }
        }

        //  `lists` still owns the nodes, so we rebuild it to take ownership
        let mut nodes: Vec<Node<T>> = Vec::new();
        // And from the lists
        for (main, adjs) in lists {
            nodes.push(main);
            for n in adjs {
                nodes.push(n);
            }
        }
        dedup(&mut nodes);

        if matriu.iter().any(Option::is_none) {
            return Err(FromListError::MalformedLists);
        }

        let values: Vec<bool> = matriu
            .into_iter()
            .map(|b| b.expect("We've already verified they're all Some"))
            .collect();

        // Check symmetry and zeros down the diagonal, just in case
        for i in 0..ordre * ordre {
            let (x, y) = idx_to_coords(i, ordre);
            let i_prime = coords_to_idx(y, x, ordre);

            if values[i] != values[i_prime] {
                return Err(FromListError::MalformedLists);
            }
            if x == y && values[i] {
                return Err(FromListError::MalformedLists);
            }
        }

        Ok(Graph {
            nodes: nodes.into_iter().collect(),
            edges: AdjMatrix { values, n: ordre },
        })
    }

    pub fn get_node(&self, x: char) -> Option<&Node<T>> {
        self.nodes.iter().find(|n| n.name == x)
    }

    /// Change the name of a node. 
    ///
    /// Returns the value of the changed node
    ///
    /// Errors if the name is already taken or if the wanted node does
    /// not exist
    pub fn change_node_name(&mut self, original: char, new: char) -> Result<&T, NodeNameChangeError> {
        if self.nodes.iter().any(|x| x.name == new) {
            return Err(NodeNameChangeError::NameAlreadyTaken);
        }
        if let Some(node) = self.nodes.iter_mut().find(|n| n.name == original) {
            node.name = new;
            Ok(&node.value)
        } else {
            Err(NodeNameChangeError::NodeNotFound)
        }
    }

    /// Change the value of a node.
    ///
    /// Returns the value of the changed node
    ///
    /// Errors only if the wanted node does not exist
    pub fn change_node_value(&mut self, node: char, mut new_val: T) -> Result<T, ()> {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.name == node) {
            mem::swap(&mut new_val, &mut node.value);
            Ok(new_val)
        } else {
            Err(())
        }
    }

    /// Returns None if the matrix is non-symmetrical or has at least in the main diagonal, or also
    /// if one of the names is repeated
    pub fn from_matrix<const N: usize>(
        names: [char; N],
        values: [T; N],
        m: [[bool; N]; N],
    ) -> Result<Self, FromMatrixError> {
        if has_duplicates(names) {
            return Err(FromMatrixError::NodesArentUnique)?;
        }
        let nodes = names.into_iter().zip(values).map(|(c, v)| Node::new(c, v)).collect();
        let edges = AdjMatrix::from_mat(m)?;

        Ok(Graph { nodes, edges })
    }

    /// Returns None if either or both of the nodes do not exist in the graph
    pub fn has_adjacency(&self, a: char, b: char) -> Option<bool> {
        let a_idx = self.nodes.iter().enumerate().find(|(_, x)| x.name == a)?.0;
        let b_idx = self.nodes.iter().enumerate().find(|(_, x)| x.name == b)?.0;
        self.edges.has_adjacency(a_idx, b_idx)
    }
    pub fn adjacent_nodes(&self, c: char) -> Option<Vec<&Node<T>>> {
        let i = self.nodes.iter().position(|x| x.name == c)?;
        let is_part_of = self.edges.get_adjacent(i);
        if is_part_of.len() != self.nodes.len() { return None; }

        Some(self.nodes.iter().zip(is_part_of).filter(|(_n, b)| **b ).map(|(n, _)| n).collect())
    }
}

#[derive(Debug, Error)]
pub enum NodeNameChangeError {
    #[error("name was already taken")]
    NameAlreadyTaken,
    #[error("no such node in the graph")]
    NodeNotFound
}
