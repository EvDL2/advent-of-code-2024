use core::iter::Iterator;
use regex::Regex;
use std::error::Error;

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    let regex_str = r"mul\(\d{1,3},\d{1,3}\)";
    let matches: Vec<String> = find_all_matches(regex_str, input);
    let ints: Vec<(i32, i32)> = matches
        .iter()
        .map(|pair| extract_integers((&pair).to_string()))
        .collect();
    let result: i32 = ints.iter().map(|(l, r)| l * r).sum();
    Ok(result)
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

fn find_all_matches(regex_str: &str, input_str: &str) -> Vec<String> {
    let regex = Regex::new(regex_str).expect("Invalid regex");
    regex
        .captures_iter(input_str)
        .map(|cap| cap.get(0).unwrap().as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            solve("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
                .unwrap();
        println!("result is: {}", result);
        assert_eq!(result, 161);
    }
}
