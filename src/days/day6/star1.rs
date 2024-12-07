use crate::utils;

fn change_dir(dir:(i8,i8)) -> (i8,i8){
    if dir.0 == 0{
        if dir.1 == 1{
            return (1,0);
        }
        if dir.1 == -1{
            return (-1,0);
        }
    }
    else if dir.0 == 1 && dir.1 == 0{
        return (0,-1);
    }
    else if dir.0 == -1 && dir.1 == 0{
        return (0,1);
    }

    return (0,0);
}

fn sum_pos(pos:(usize,usize),dir:(i8,i8)) -> (usize,usize){
    let num1: usize;
    let num2:usize;

    if dir.0.is_negative() {
        num1 = pos.0.checked_sub(dir.0.unsigned_abs() as usize).unwrap_or(usize::MAX);
    } else {
        num1 = pos.0 + dir.0 as usize;
    }
    if dir.1.is_negative() {
        num2 = pos.1.checked_sub(dir.1.unsigned_abs() as usize).unwrap_or(usize::MAX);
    } else {
        num2 = pos.1 + dir.1 as usize;
    }

    return (num1,num2);
}

#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day6\\input_day6.txt"){
    //match utils::read_file_content("src\\days\\day6\\test.txt"){
        Ok(content) => {
            let mut matrix: Vec<Vec<char>> = Vec::new(); //Vector for every rulee;
            let mut police_position: (usize,usize) =  (0,0);
            let mut final_result:usize = 1;
            //let mut is_ended: bool = false;
            let police:char = '^';
            let walked:char = 'X'; //Police pass in this position
            let obstacle:char = '#';

            let mut line_row:usize = 0;
            for line in content.lines(){
                let chars: Vec<char> = line.chars().collect();
                
                if let Some(pos) = chars.iter().position(|&x|    x == '^'){
                    police_position = (line_row,pos);
                }

                matrix.push(chars);     
                line_row+=1;                       
            }


            let mut dir:(i8,i8) = (-1,0);
            loop{
                let new_pos:(usize,usize) = sum_pos(police_position, dir);

                //Check if is finished
                if new_pos.0 == usize::MAX || new_pos.1 == usize::MAX || new_pos.0 >= matrix.len() || new_pos.1 >= matrix[0].len(){
                    break;
                }
                else if matrix[new_pos.0][new_pos.1] == obstacle{
                    dir = change_dir(dir);
                }
                else if matrix[new_pos.0][new_pos.1] == walked{
                    matrix[police_position.0][police_position.1] = walked;
                    matrix[new_pos.0][new_pos.1] = police;
                    police_position = new_pos;
                }
                else{
                    matrix[police_position.0][police_position.1] = walked;
                    matrix[new_pos.0][new_pos.1] = police;
                    police_position = new_pos;
                    final_result += 1;
                }             
            }

            println!("Day 6, star 1: {}",final_result);
        }
        Err(_e) => {
            print!("Error detected: {}",_e);
        }
    }
}
