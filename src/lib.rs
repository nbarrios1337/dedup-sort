/// dedup_sort built with functionality available from std
pub fn std_dedup_sort<T: Ord>(v: &mut Vec<T>) {
    v.sort();
    v.dedup();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = [3, 2, 2, 1].to_vec();
        std_dedup_sort(&mut v);
        assert_eq!(v, [1, 2, 3])
    }
}
