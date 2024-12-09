use std::error::Error;
use day_02::part1::solve;

fn main() -> Result<(), Box<dyn Error>> {
    
    let file = include_str!("../../inputs/input1.txt");
    let result = solve(file)?;
    println!("{}", result);
    Ok(())
}