use crate::utils;
use regex::Regex;

//Count the words XMAS and SMAX in a string
fn count_words_string(line:&str) -> usize{
    let re_xmas: Regex = Regex::new(r"XMAS").unwrap();
    let re_samx: Regex = Regex::new(r"SAMX").unwrap();

    let count_words: usize= re_xmas.find_iter(line).count() + re_samx.find_iter(line).count();

    return count_words;
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
                
                //Search horizontal words
                total_words += count_words_string(line);

                //Build the matrix
                matrix.push(chars);
            }

            //Search vertial words only for rectangular ones
            for i in 0..matrix.len(){
                let mut col: String = String::from("");

                for j in 0..matrix[i].len(){
                    col.push(matrix[j][i]);

                }
                total_words += count_words_string(&col);
            }

            
            //Diagonals
            for i in 0..matrix.len(){
                let mut diagonal: String = String::from("");
                for j in 0..matrix[i].len(){
                    if i < matrix.len() && j < matrix.len() && i+j < matrix.len(){
                        diagonal.push(matrix[i+j][j]);
                    }
                }
                total_words += count_words_string(&diagonal);
            }
            for i in 0..matrix.len(){
                let mut diagonal: String = String::from("");
                for j in 0..matrix[i].len(){
                    if i < matrix.len() && j+1 < matrix.len() && i+j+1 < matrix.len(){
                        diagonal.push(matrix[j][1+j+i]);
                    }

                }
                total_words += count_words_string(&diagonal);
            }

            //Reverse diagonals
            for i in 0..matrix.len(){
                let mut diagonal: String = String::from("");
                for j in 0..matrix[i].len(){
                    if j+i < matrix.len() && j < matrix[i].len(){
                        diagonal.push(matrix[j+i][matrix[i].len() - 1 - j]);
                    }
                }
                total_words += count_words_string(&diagonal);
            }
            for i in 0..matrix.len(){
                let mut diagonal: String = String::from("");
                for j in 0..matrix[i].len(){
                    if i > 0 && j < matrix.len() && j + i < matrix[j].len(){
                        diagonal.push(matrix[j][matrix[i].len() - 1 - i - j]);
                    }
                }
                total_words += count_words_string(&diagonal);
            }

            
            println!("Day 3, star 1: {}",total_words);
        }
        Err(_e) => {
            print!("Error detected: {}",_e);
        }
    }
}