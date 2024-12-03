use crate::utils;
use regex::Regex;

#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day3\\input_day3.txt"){
        Ok(content) => {
            let mut result: usize = 0;
            let re_general: Regex = Regex::new(r"(mul[(]\d{1,3}[,]\d{1,3}[)])|do\(\)|don't\(\)").unwrap();
            let re_get_numbers: Regex = Regex::new(r"\b\d{1,3}\b").unwrap();
            let mut mul_enable: bool = true;

            // Find all matches in the string
            for mat in re_general.find_iter(&content.as_str()) {
                if mat.as_str() == "do()"{
                    mul_enable = true;
                    continue;
                }
                else if mat.as_str() == "don't()"{
                    mul_enable = false;
                    continue;
                }
                else if mul_enable{
                    let numbers: Vec<usize> = re_get_numbers.find_iter(mat.as_str()).map(|x| x.as_str().parse::<usize>().unwrap()).collect();
                    result += numbers[0] * numbers[1];
                }
                
                //println!("Found match: {} -> numbers:{:?}", mat.as_str(),numbers);

            }

            println!("Day 2, star 1: {}",result)
        }
        Err(_e) => {
            print!("Error detected: {}",_e);
        }
    }
}