use core::iter::Iterator;
use regex::Regex;
use std::error::Error;

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    let regex_str = r"mul\(\d{1,3},\d{1,3}\)";
    let do_str = r"do\(\)";
    let dont_str = r"don't\(\)";
    let (do_matches, do_position) = find_all_matches(&do_str, input);
    let (dont_matches, dont_position) = find_all_matches(&dont_str, input);
    let (regex_matches, regex_position) = find_all_matches(&regex_str, input);
    let mut onoff = concatenate_vectors_with_zero(&do_position, &dont_position);
    onoff.sort();
    println!("{:?}", &do_position);
    println!("{:?}", &dont_position);
    println!("{:?}", &onoff);

    let ints: Vec<(i32, i32)> = regex_matches
        .iter()
        .map(|pair| extract_integers((&pair).to_string()))
        .collect();
    
    // let result: i32 = ints.iter().map(|(l, r)| l * r).sum();
    Ok(0)
}

fn concatenate_vectors_with_zero(vec1: &[usize], vec2: &[usize]) -> Vec<usize> {
    // Concatenate vectors from references into a mutable vector
    // Add a zero as the first element
    let zero_vec: Vec<usize> = vec![0];
    let mut concatenated_vec = Vec::with_capacity(1 + vec1.len() + vec2.len());
    concatenated_vec.extend_from_slice(&zero_vec);
    concatenated_vec.extend_from_slice(vec1);
    concatenated_vec.extend_from_slice(vec2);
    concatenated_vec
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
