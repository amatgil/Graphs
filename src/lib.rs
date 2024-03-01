use std::{cmp, error::Error};

use thiserror::Error;
use utils::{coords_to_idx, has_duplicates};

mod utils;

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
    /// Takes a list of lists and returns the corresponding adjacency matrix.  
    //pub fn from_lists(lists: &[&[Node<T>]]) -> Result<Self, FromListError> {
    //    let n = cmp::max(
    //        lists.iter().fold(0, |acc, l| cmp::max(acc, l.len())),
    //        lists.len(),
    //    );
    //}
    /// Panics if either a or b are out of bounds
    fn has_adjacency(&self, a: usize, b: usize) -> Option<bool> {
        if a > self.n || b > self.n {
            None
        } else {
            // Matrix is symmetric, a and b can be swapped
            Some(self.values[coords_to_idx(a, b, self.n)])
        }
    }
}

/// Basic Node type, which the graph connects. Note that changing its name or its value (if any)
/// will NOT change the structure of the graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub name: char,
    pub value: Option<T>,
}

impl<T> Node<T> {
    /// Convenience, normal constructor may also be used without problems
    pub fn new(name: char, value: Option<T>) -> Self {
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
}

impl<T> Graph<T> {
    pub fn from_list(lists: &[(Node<T>, &[Node<T>])]) -> Result<Self, FromListError> {
        todo!()
    }

    /// Returns None if the matrix is non-symmetrical or has at least in the main diagonal, or also
    /// if one of the names is repeated
    pub fn from_matrix<const N: usize>(
        names: [char; N],
        m: [[bool; N]; N],
    ) -> Result<Self, FromMatrixError> {
        if has_duplicates(names) {
            return Err(FromMatrixError::NodesArentUnique)?;
        }
        let nodes = names.into_iter().map(|c| Node::new(c, None)).collect();
        let edges = AdjMatrix::from_mat(m)?;

        Ok(Graph { nodes, edges })
    }

    /// Returns None if either or both of the nodes do not exist in the graph
    pub fn has_adjacency(&self, a: char, b: char) -> Option<bool> {
        let a_idx = self.nodes.iter().enumerate().find(|(_, x)| x.name == a)?.0;
        let b_idx = self.nodes.iter().enumerate().find(|(_, x)| x.name == b)?.0;
        self.edges.has_adjacency(a_idx, b_idx)
    }
}
