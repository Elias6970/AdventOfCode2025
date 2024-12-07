use crate::utils;
use itertools::Itertools;



fn apply_operator(a: usize, b: usize, operator: char) -> usize {
    match operator {
        '*' => a * b,
        '+' => a + b,
        _ => panic!("Unsupported operator"),
    }
}

fn calculate(numbers:&Vec<usize>,result:usize,operators:&Vec<char>) -> Result<usize,Vec<char>>{
    let mut actual_result:usize = numbers[0];
    for i in 1..numbers.len(){
        //println!("Before:{},op->{}",actual_result,operators[i-1]);
        actual_result = apply_operator(actual_result, numbers[i], operators[i-1]);
        //println!("After:{}\n",actual_result);

        if result == actual_result{
            return Ok(result);
        }
        else if result < actual_result{
            return Err(operators.iter().take(i).cloned().collect());

        }
    }
    return Err(operators.iter().cloned().collect());
}


fn all_permutations(length: usize) -> Vec<Vec<char>>{
    let repeated_chars = vec![vec!['*', '+']; length];

    // Generate all permutations
    let permutations: Vec<Vec<char>> = repeated_chars
        .into_iter()
        .multi_cartesian_product()
        .collect();

    return permutations;
}


// Helper function to check if a vector starts with the given sequence
fn starts_with_sequence(vec: &Vec<char>, sequence: &Vec<char>) -> bool {
    vec.iter().take(sequence.len()).eq(sequence.iter())
}

#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day7\\input_day7.txt"){
    //match utils::read_file_content("src\\days\\day7\\test.txt"){
        Ok(content) => {
            let mut total_result:usize = 0;

            for line in content.lines(){
                if let Some((first_part,second_part)) = line.split_once(": "){
                    if let Ok(result) = first_part.parse::<usize>(){
                        let numbers:Vec<usize> = second_part.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();

                        //println!("R:{}->Num:{:?}",result,numbers);
                        println!("{}",line);
                        //let mul: char = '*';
                        //let sum: char = '+';
                        let mut all_permutations: Vec<Vec<char>> = all_permutations(numbers.len()-1);

                        loop{
                            //println!("Operators:{:?}",all_permutations);
                            if all_permutations.is_empty(){
                                break;
                            }

                            match calculate(&numbers, result, &all_permutations[0]){
                                Ok(result) => {
                                    total_result += result;
                                    break;
                                },
                                Err(wrong_operators) => all_permutations.retain(|x|{
                                    !(starts_with_sequence(x, &wrong_operators))
                                }),
                            }
                        }
                    }
                }
            }

            println!("Day 7, star 1:{}",total_result);
        }
        Err(_e) => {
            println!("Error detected: {}",_e);
        }
    }
}
