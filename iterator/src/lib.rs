#[cfg(test)]
mod tests {
    #[test]
    fn run_iter_next() {
        let v = vec![1, 2, 3];
        let mut v_iter = v.iter();

        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);
    }

    #[test]
    fn run_iter_sum() {
        let v = vec![1, 2, 3];
        let v_iter = v.iter();
        let sum: i32 = v_iter.sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn run_iter_map() {
        let v = vec![1, 2, 3];
        let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}
