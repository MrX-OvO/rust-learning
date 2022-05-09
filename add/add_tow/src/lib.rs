pub fn add_tow(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_add_tow() {
        assert_eq!(3, add_tow(1));
    }
}