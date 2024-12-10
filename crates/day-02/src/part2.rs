use core::iter::Iterator;
use std::error::Error;

pub fn solve(input: &str) -> Result<u32, Box<dyn Error>> {
    let mat = parse_input(input);
    let lower_bound: i32 = 1; // lower bound of the difference
    let upper_bound: i32 = 3; // upper bound of the difference
    
    let mut total = 0;

    'outer: for row in &mat {
        let res = diff_vec_meets_conditions(&get_diff_vec(&row), lower_bound, upper_bound);
        if res == true {
            total += 1;
            continue;
        }
        for position in 0..row.len() {
            let mut row = row.clone();
            row.remove(position);
            let diff = get_diff_vec(&row);
            let res = diff_vec_meets_conditions(&diff, lower_bound, upper_bound);
            if res == true {
                total += 1;
                continue 'outer;
            }
        }
    }
    Ok(total)
}

fn get_diff_vec(v: &Vec<u32>) -> Vec<i32> {
    // Return a vector of differences between consecutive elements
    let v: Vec<i32> = v.iter().map(|&x| x as i32).collect();
    v.windows(2).map(|x| x[1] - x[0]).collect()
}

fn diff_vec_meets_conditions(v: &Vec<i32>, lower_bound: i32, upper_bound: i32) -> bool {
    let bounds = v
        .iter()
        .map(|&diff| diff.abs() >= lower_bound && diff.abs() <= upper_bound)
        .all(|x| x);
    let monotonic_increase = v.iter().all(|&x| x >= 0);
    let monotonic_decrease = v.iter().all(|&x| x <= 0);
    bounds && (monotonic_increase || monotonic_decrease)
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    // Iterate over lines of the text
    // Split the line by whitespace
    // Parse each element of the text into a u32
    // Collect the iterator into the return type
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| {
                    x.parse::<u32>()
                        .expect("Elements of the string must be integers")
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(
            "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9",
        )
        .unwrap();
        println!("result is: {}", result);
        assert_eq!(result, 4);
    }
}
