use std::error::Error;

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
    values: Vec<bool>, // Only the UPPER half is complete, the other half is  full of
    // zeros becuase it's repeated option
    n: usize,
}

impl AdjMatrix {
    pub fn from_mat<const N: usize>(m: [[bool; N]; N]) -> Result<Self, FromMatrixError> {
        // Check for symmetry:
        for y in 0..N {
            for x in 0..N {
                if m[x][y] != m[y][x] { return Err(FromMatrixError::IsNotSymmetric);}
            }
            if m[y][y] { return Err(FromMatrixError::NonEmptyDiagonal); }
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
}

/// Basic Node type, which the graph connects. Note that changing its name or its value (if any)
/// will NOT change the structure of the graph.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node<T> {
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
enum FromMatrixError {
    #[error("input matrix was not symmetrical")]
    IsNotSymmetric,
    #[error("the names of the nodes contain duplicates")]
    NodesArentUnique,
    #[error("the main diagonal containes one or more 'true's")]
    NonEmptyDiagonal,
}



impl<T> Graph<T> {
    pub fn from_list(v: &[Node<T>]) {
        todo!()
    }

    /// Returns None if the matrix is non-symmetrical or has at least in the main diagonal, or also
    /// if one of the names is repeated
    pub fn from_matrix<const N: usize>(names: [char; N], m: [[bool; N]; N]) -> Result<Self, Box<dyn Error>> {
        if has_duplicates(names) { return Err(FromMatrixError::NodesArentUnique)?; }
        let nodes = names.into_iter().map(|c| Node::new(c, None)).collect();
        let edges = AdjMatrix::from_mat(m)?;

        Ok(Graph { nodes, edges })
    }
    /// Returns None if either or both of the nodes do not exist in the graph
    pub fn has_adjacency(&self, a: char, b: char) -> Option<bool> {
        let a_idx = self.nodes.iter().enumerate().find(|(i, x)| x.name == a )?.0;
        let b_idx = self.nodes.iter().enumerate().find(|(i, x)| x.name == b )?.0;
        self.edges.has_adjacency(a_idx, b_idx)
    }
}
