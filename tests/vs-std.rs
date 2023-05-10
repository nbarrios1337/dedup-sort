mod common;

/// This should never, ever fail.
/// Literally copied tested function's code over.
#[test]
fn correct_std() {
    let mut v: Vec<i32> = common::randomized_vec(10000);
    let mut correct = v.clone();

    dedup_sort::std_dedup_sort(&mut v);
    common::known_good_dedup_sort(&mut correct);

    assert_eq!(v, correct);
}
