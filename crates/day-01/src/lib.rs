pub fn solve() -> u64 {
    println!("I'm in the function for solving Day 1!");
    1 + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve();
        assert_eq!(result, 3);
    }
}
