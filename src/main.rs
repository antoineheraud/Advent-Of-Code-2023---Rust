use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::char;

// Main for Day 1 part 2
fn main() {
    let file_path = "example_part_2.txt";
    let file_path = "trebuchet_input.txt";
    let file = File::open(file_path)
        .expect("Should have been able to read the file");
    let reader_lines = BufReader::new(file).lines();
    let mut sum = 0;
    for line in reader_lines {
        let mut first_digit = -1;
        let mut last_digit = 0;
        if let Ok(ip) = line {
            println!("{}",ip);
            let mut replaced_line = ip.clone();
            let mut replaced_line_vec: Vec <String> = vec![];
            replaced_line_vec.push(replaced_line.replace("one","o1e"));
            replaced_line_vec.push(replaced_line.replace("two","t2o"));
            replaced_line_vec.push(replaced_line.replace("three","th3ee"));
            replaced_line_vec.push(replaced_line.replace("four","fo4ur"));
            replaced_line_vec.push(replaced_line.replace("five","fi5e"));
            replaced_line_vec.push(replaced_line.replace("six","s6x"));
            replaced_line_vec.push(replaced_line.replace("seven","se7en"));
            replaced_line_vec.push(replaced_line.replace("eight","ei8ht"));
            replaced_line_vec.push(replaced_line.replace("nine","n9ne"));
            // Accross the 9 line possibility search last and first number            
            let mut first_position = 100000000;
            let mut first_digit = -1;
            let mut last_position = -1;
            let mut last_digit = -1;
            
            for r_l in 0..9 {
                let chars = replaced_line_vec[r_l].chars();
                let mut position = 0;
                for c in chars {
                    if c.is_digit(10) {
                        let c_digit = c.to_digit(10)
                            .expect("chars not a digit") as i32;
                       
                        if position < first_position {
                            first_digit = c_digit;
                            first_position = position;
                        }
                        if position > last_position {
                            last_digit = c_digit;
                            last_position = position;
                        }
                    }
                    position += 1;
                }
                //println!("{}", replaced_line_vec[r_l]);
                //println!("first position = {} and last position = {}", first_position, last_position);
            }
            //println!("{}{}", first_digit,last_digit); 
            //println!("{:?}", replaced_line_vec);
            // SUM 
            sum += first_digit*10;
            sum += last_digit;
            println!("SUM = {}", sum);

            
            // Not used but kept as reference for function use
            /*println!("{}", replaced_line);
            match trebuchet(replaced_line) {
                Ok(line_sum) => sum += line_sum,
                Err(e) => println!("{}",e),
            }*/
        }
    }
}

fn trebuchet(line: String) -> Result<i32, &'static str> {
    let mut first_digit = -1;
    let mut last_digit = 0;
    let chars = line.chars();
    let mut sum = 0;
    for c in chars {
        if c.is_digit(10) {
            let c_digit = c.to_digit(10)
                .expect("chars not a digit") as i32;
            if first_digit == -1 {
                first_digit = c_digit;
            }
            last_digit = c_digit;
        }
    }
    if first_digit != -1 {
        sum += first_digit*10;
        sum += last_digit;
    }
    else {
        return Err("No digit found in this line");
    }
    Ok(sum)
}
        
fn main_part_1() {
    let file_path = "trebuchet_input.txt";
    //let file_path = "example.txt";
    //println!("{file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    //println!("With text:\n{contents}");

    let file = File::open(file_path)
        .expect("Should have been able to read the file");
    
    let reader_lines = BufReader::new(file).lines();

    let mut sum =0;

    for line in reader_lines {
        let mut first_digit = -1;
        let mut last_digit = 0;
        if let Ok(ip) = line {
            println!("{}", ip);
            let mut chars = ip.chars();
            for c in chars {
                if c.is_digit(10) {
                    let c_digit = c.to_digit(10)
                        .expect("chars not a digit") as  i32;
                    if first_digit == -1 {
                        first_digit = c_digit;
                    }
                    last_digit = c_digit;
                }
            }
            println!("First digit = {}", first_digit);
            println!("Last digit = {}", last_digit);
            if first_digit != -1 {
                sum += first_digit*10;
            }
            sum += last_digit;
            println!("SUM = {}", sum);
        }
    }
}
