use std::collections::BTreeSet;

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
