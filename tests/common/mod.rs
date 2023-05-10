// Using older file namign convention
#![allow(dead_code)]

use rand::Rng;

/// Generates a vector with `size` elements, each randomly generated
pub(crate) fn randomized_vec(size: usize) -> Vec<i32> {
    let mut v = vec![0; size];
    rand::thread_rng().fill(&mut v[..]);
    v
}

/// Generates a vector with `size` elements, each in the range [`start`..`end`)
pub(crate) fn randomized_vec_in_range(size: usize, start: i32, end: i32) -> Vec<i32> {
    let range = rand::distributions::Uniform::new(start, end);
    std::iter::repeat_with(|| rand::thread_rng().sample(range))
        .take(size)
        .collect()
}

/// Sorts then deduplicates the elements of the vector `v`
///
/// There's no optimization here, just function calls that get the job done
pub(crate) fn known_good_dedup_sort(v: &mut Vec<i32>) {
    v.sort();
    v.dedup();
}
