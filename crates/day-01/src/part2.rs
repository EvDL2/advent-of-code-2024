use std::error::Error;

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    let (mut left, mut right) = parse_input(input);

    // Initialize a result variable
    let mut total: i32 = 0;
    
    // Sort both columns
    left.sort();
    right.sort();

    for element in left {
        let num_element_in_right = right.iter().filter(|&i| *i == element).count() as i32;
        #[cfg(debug_assertions)]
        {
            dbg!("For {} in left, found {} instances in right", element, num_element_in_right);
        }
        total += element * num_element_in_right;
    }

    Ok(total)
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Loop over the lines, parsing their elements into integers and pushing them into column vectors
    for line in input.lines() {
        let mut row = line.split_whitespace();
        left.push(row.next().unwrap().parse::<i32>().expect("Value must be an integer"));
        right.push(row.next().unwrap().parse::<i32>().expect("Value must be an integer"));
    }

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
        assert_eq!(result, 31);
    }
}
