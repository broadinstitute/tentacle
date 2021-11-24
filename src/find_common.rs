use std::cmp::Ordering;

pub(crate) fn find_common_sorted<T: Ord>(vec1: &[T], vec2: &[T]) -> Vec<(usize, usize)> {
    let mut indices = Vec::<(usize, usize)>::new();
    let mut index1 = 0_usize;
    let mut index2 = 0_usize;
    while let
    (Some(item1), Some(item2)) = (vec1.get(index1), vec2.get(index2))
    {
        match item1.cmp(item2) {
            Ordering::Less => {
                index1 += 1
            }
            Ordering::Equal => {
                indices.push((index1, index2));
                index1 += 1;
                index2 += 1;
            }
            Ordering::Greater => {
                index2 += 1
            }
        }
    }
    indices
}