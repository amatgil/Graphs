use core::hash;
use std::{cmp, collections::HashMap, fmt::Debug, io, path::PathBuf};

use crate::{traversal::TraversalError, Graph, Node};

use ppmitzador::{self, Coord, ImagePBM, ImagePPM, Pixel, PpmFormat};

#[derive(Debug, thiserror::Error)]
pub enum DrawingError {
    #[error("graph was empty")]
    EmptyGraph,
    #[error("errored while running BFS")]
    BFS(TraversalError),
    #[error("errored while saving the resulting file")]
    FileSystem(io::Error),
}

impl<T: cmp::Eq + hash::Hash > Graph<T>  {
    pub fn draw_and_save_to_file(
        &self,
        filepath: impl Into<PathBuf>,
        width: usize,
        height: usize,
    ) -> Result<(), DrawingError> {
        let color_table = vec![
            Pixel::new(0x03, 0x3F, 0x63),
            Pixel::new(0x28, 0x66, 0x6E),
            Pixel::new(0xDE, 0x63, 0x9A),
            Pixel::new(0xFF, 0xED, 0x65),
            Pixel::new(0xEA, 0xD9, 0x4C),
            Pixel::new(0xBF, 0xB7, 0xB6),
        ];
        let mut img = ImagePPM::new(width, height, Pixel::BLACK);
        let height = ((height as f32)*0.7) as usize;
        let width = ((width as f32)*0.7) as usize;

        if self.is_tree().0 {
            println!("Drawing a tree");
            let (nodes, arestes, max_dist) = self
                .bfs(self.nodes.get(0).ok_or(DrawingError::EmptyGraph)?)
                .map_err(|e| DrawingError::BFS(e))?;

            let mut nodes_by_dist: Vec<Vec<&Node<T>>> = vec![]; 

            dbg!(&nodes);
            for curr_d in 0..=max_dist {
                let nodes_at_d: Vec<_> = nodes
                    .iter()
                    .filter(|(_n, d)| *d == curr_d)
                    .map(|(n, _)| *n)
                    .collect();
                nodes_by_dist.push(nodes_at_d);
            }

            let levels = nodes_by_dist.len();
            let layer_height = height / levels;

            let mut final_node_positions: Vec<(&Node<T>, (usize, usize))> = vec![];

            let spacing = width / nodes_by_dist.iter().map(|l| l.len()).max().unwrap();
            for (i, nodes_in_level) in nodes_by_dist.iter().enumerate() {
                let y_position = height - layer_height*i;
                //let spacing = width / nodes_in_level.len();
                let start = width / 2 - (spacing*(nodes_in_level.len() - 1)/2);
                for (j, n) in nodes_in_level.iter().enumerate() {
                    let x = start + spacing*j;
                    dbg!(start, spacing, j, n, x);
                    final_node_positions.push((n, (x, y_position)));
                }

            }


            dbg!(&final_node_positions);
            for (i, (_n, (x, y))) in final_node_positions.iter().enumerate() {
                img.draw_circle(Coord { x: *x, y: *y }, 30, color_table[i % color_table.len()]);
            }

            for (u, (ux, uy)) in &final_node_positions {
                for (v, (vx, vy)) in &final_node_positions {
                    let u_coord = Coord::new(*ux, *uy);
                    let v_coord = Coord::new(*vx, *vy);

                    if arestes.contains(&(u, v)) || arestes.contains(&(v, u)) {
                        img.draw_line_with_thickness(u_coord, v_coord, Pixel::WHITE, 3)
                    }
                }
            }

        } else {
            todo!("Non-tree implementation not yet made")
        }

        img.save_to_file(filepath).map_err(|e| DrawingError::FileSystem(e))?;

        Ok(())
    }
}
