use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

pub fn coords_to_idx(x: usize, y: usize, w: usize) -> usize {
    x + w * y
}

pub fn idx_to_coords(i: usize, w: usize) -> (usize, usize) {
    (i % w, i / w)
}

pub fn has_duplicates<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Ord,
{
    let mut uniq = BTreeSet::new();
    !iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn dedup<T>(v: &mut Vec<T>)
where
    T: Hash + Eq + std::fmt::Debug,
{
    let mut set = HashSet::new();
    let mut idxs = Vec::new();

    for i in 0..v.len() {
        if !set.insert(&v[i]) { idxs.push(i); }
    }

    // Right to left, because it mslides them back 
    for i in idxs.into_iter().rev() {
        v.remove(i); 
    }
}
