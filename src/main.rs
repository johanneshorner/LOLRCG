mod cli;
mod generator;

fn main() {
    cli::run();
}

fn test2() -> i32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test1() {
        assert_eq!(5, test2());
    }
}
