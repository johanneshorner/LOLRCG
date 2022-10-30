fn test1() -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        assert_eq!(2, test1());
    }
}
