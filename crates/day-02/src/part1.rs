use core::iter::Iterator;
use std::error::Error;

pub fn solve(input: &str) -> Result<u32, Box<dyn Error>> {
    let mat = parse_input(input);
    let diff_lower_bound = 1;
    let diff_upper_bound = 3;

    // let t_vec = mat[0];
    // let diff_mat = t_vec
    //     .windows(2)
    //     .map(|x| {
    //         let tmp = x[1] - x[0];
    //         tmp >= diff_lower_bound && tmp >= diff_upper_bound
    //     })
    //     .collect::<Vec<bool>>();

    let total = mat.iter().map(|row| all_within_bound(&row, diff_lower_bound, diff_upper_bound)).count();
    let total = u32::try_from(total).unwrap();

    Ok(total)
}

fn all_within_bound(v: &Vec<u32>, diff_lower_bound: u32, diff_upper_bound: u32) -> bool {
    v.windows(2)
        .map(|x| {
            let diff= x[1] - x[0];
            diff >= diff_lower_bound && diff >= diff_upper_bound
        })
        .all(|x| x)
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
        assert_eq!(result, 2);
    }
}
