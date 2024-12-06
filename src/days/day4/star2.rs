use crate::utils;
use regex::Regex;

//Know if the words XMAS and SMAX in a string
fn is_a_word_in_string(line:&str) -> bool{
    let re_xmas: Regex = Regex::new(r"MAS").unwrap();
    let re_samx: Regex = Regex::new(r"SAM").unwrap();

    return re_xmas.find_iter(line).count() > 0 || re_samx.find_iter(line).count() > 0;
}


#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day4\\input_day4.txt"){
    //match utils::read_file_content("src\\days\\day4\\test.txt"){
        Ok(content) => {
            let mut matrix: Vec<Vec<char>> = Vec::new();
            let mut total_words: usize = 0;

            //Create the matrix and found horizontal words
            for line in content.lines(){
                let chars:Vec<char> = line.chars().collect();
                
                //Build the matrix
                matrix.push(chars);
            }

            for i in 0..matrix.len(){
                for j in 0..matrix[i].len(){
                    if i+2 < matrix.len() && j+2 < matrix.len(){
                        let first_word = format!("{}{}{}",matrix[i][j],matrix[i+1][j+1],matrix[i+2][j+2]);
                        if is_a_word_in_string(&first_word){
                            let second_word = format!("{}{}{}",matrix[i][j+2],matrix[i+1][j+1],matrix[i+2][j]);

                            if is_a_word_in_string(&second_word){
                                total_words += 1;
                            }
    
                        }
                    }
                }
            }

            println!("Day 4, star 1: {}",total_words);
        }
        Err(_e) => {
            print!("Error detected: {}",_e);
        }
    }
}