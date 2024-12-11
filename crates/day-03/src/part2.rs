use core::iter::Iterator;
use regex::Regex;
use std::error::Error;

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    // Get the matches and their positions
    let regex_str = r"mul\(\d{1,3},\d{1,3}\)";
    let do_str = r"do\(\)";
    let dont_str = r"don't\(\)";
    let (_do_matches, do_positions) = find_all_matches(&do_str, input);
    // Append zero to the front since the challenge states that mul instructions are enabled at the beginning
    let do_positions = concatenate_vectors(&vec![0], &do_positions);
    let (_dont_matches, dont_positions) = find_all_matches(&dont_str, input);
    let (regex_matches, regex_positions) = find_all_matches(&regex_str, input);

    let mut total = 0;

    // Loop over regex matches
    // For each match, find the closest instances of do() and don't() that are smaller than position of the mul expression
    // If the instance of do() is greater than the instance of don't() (aka, do() happened more recently), add to the total
    // NOTE: This might fail if there are no instances of don't() in the input
    for (idx, pair) in regex_matches.iter().enumerate() {
        let (l, r) = extract_integers((&pair).to_string());
        let pair_position = regex_positions[idx];
        let closest_smaller_do = closest_smaller_element(do_positions.clone(), pair_position);
        let closest_smaller_dont = closest_smaller_element(dont_positions.clone(), pair_position);
        if closest_smaller_do > closest_smaller_dont {
            total += l * r;
        }
    }

    Ok(total)
}

fn concatenate_vectors(vec1: &[usize], vec2: &[usize]) -> Vec<usize> {
    // Concatenate vectors from references into a mutable vector
    let mut concatenated_vec = Vec::with_capacity(vec1.len() + vec2.len());
    concatenated_vec.extend_from_slice(vec1);
    concatenated_vec.extend_from_slice(vec2);
    concatenated_vec
}

fn closest_smaller_element(mut vec: Vec<usize>, target: usize) -> Option<usize> {
    // Get the element closest to, but smaller than, the target
    vec.sort_unstable();
    vec.into_iter().rev().find(|&x| x < target)
}

fn extract_integers(input: String) -> (i32, i32) {
    let pair = match input.strip_prefix("mul(").and_then(|s| s.strip_suffix(")")) {
        Some(x) => x,
        None => "0,0",
    };
    let (l, r) = pair.split_once(",").unwrap();
    let l = l.parse::<i32>().expect("Must be integer");
    let r = r.parse::<i32>().expect("Must be integer");
    (l, r)
}

fn find_all_matches(regex_str: &str, input_str: &str) -> (Vec<String>, Vec<usize>) {
    let regex = Regex::new(regex_str).expect("Invalid regex");
    let matches: Vec<String> = regex
        .captures_iter(input_str)
        .map(|cap| cap.get(0).unwrap().as_str().to_string())
        .collect();
    let positions: Vec<usize> = regex
        .captures_iter(input_str)
        .map(|cap| cap.get(0).unwrap().start())
        .collect();
    (matches, positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            solve("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
                .unwrap();
        println!("result is: {}", result);
        assert_eq!(result, 48);
    }
}
