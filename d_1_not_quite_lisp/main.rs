use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;

fn main() {
    let file_path = "puzzle_input.txt";
    println!("{file_path}");
    let contents = fs::read_to_string("puzzle_input.txt")
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    let chars = contents.chars();

    let mut sum = 0;
    let mut it = 0;
    let mut pos = -1;
    let mut first = 0;
    for c in chars {
        it +=1;
        if c == '(' {
            sum += 1;
        }
        else if c == ')' {
            sum -= 1;
        }
        if sum == -1 {
            if first == 0 {
                pos = it;
            }
            first = 1;
        }
    }
    
    println!("SUM RESULT = {sum}");
    println!("POSITION = {pos}");

}


fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
