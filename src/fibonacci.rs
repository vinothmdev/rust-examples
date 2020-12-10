// This is to test unit testing in rust
fn fibonacci_itr(n: usize) -> usize {
    let mut current = 1;
    let mut n_1 = 1;
    for _ in 1..n {
        let temp = n_1;
        n_1 = current;
        current = current + temp; 
    } 
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_itr() {
        assert_eq!(fibonacci_itr(5), 8);
    }
}