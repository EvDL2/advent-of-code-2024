use std::error::Error;

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    println!("I'm in the function for solving Day 1!");
    let (mut left, mut right) = parse_input(input);
    
    // Sort both columns
    left.sort();
    right.sort();

    // Iterate over left and right, comparing elements
    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(result)
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut rows: Vec<Vec<&str>> = Vec::new();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Accumulate rows line by line as Vec<&str>
    for line in input.lines() {
        let row = line.split_whitespace().collect::<Vec<&str>>();
        rows.push(row);
    }

    // Loop over row vectors, parsing their elements into integers and pushing them to column vectors
    for row in rows {
        // Skip rows that do not have 2 values
        if row.len() >= 2 {
            left.push(row[0].parse::<i32>().expect("Value must be an integer"));
            right.push(row[1].parse::<i32>().expect("Value must be an integer"));
        }
    }

    // Alternatively, with hindsight
    // for line in input.lines() {
    //     let mut row = line.split_whitespace();
    //     left.push(row.next().unwrap().parse::<i32>().expect("Value must be an integer"));
    //     right.push(row.next().unwrap().parse::<i32>().expect("Value must be an integer"));
    // }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve("3   4
4   3
2   5
1   3
3   9
3   3").unwrap();
        println!("result is: {}", result);
        assert_eq!(result, 11);
    }
}
