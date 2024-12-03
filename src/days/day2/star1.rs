use crate::utils;

#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day2\\input_day2.txt"){
        Ok(content) => {
            let mut total_safes: u32 = 0;

            for line in content.lines(){
                let vect: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();


                let desc:bool = vect[0] > vect[1];

                for i in 0..vect.len(){
                    if i == vect.len() - 1 {total_safes+=1;}
                    
                    //Descendente, de mayor a menor
                    else if desc && (vect[i] <= vect[i+1] || (vect[i] - vect[i+1]).abs() > 3){
                        break;
                    }
                    else if !desc && (vect[i] >= vect[i+1] || (vect[i] - vect[i+1]).abs() > 3){
                        break;
                    }
                }
            }

            println!("Day 2, star 1: {}",total_safes)
        }
        Err(_e) => {
            print!("Error detected {}",_e);
        }
    }
}