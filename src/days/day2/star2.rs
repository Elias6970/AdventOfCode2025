use crate::utils;

fn is_desc(vect:&Vec<i32>) -> bool{
    let mut num_desc: usize = 0;
    for i in 0..vect.len()-1{
        if vect[i] > vect[i+1]{
            num_desc += 1;
        }
    }
    return num_desc >= (vect.len() - num_desc);
}

fn is_safe(vect:&Vec<i32>,desc:bool,deleted_id:i32) -> bool{
    for i in 0..vect.len(){
        let actual_id: usize = if i as i32 == deleted_id {i+1} else {i};
        let next_id: usize = if i as i32 == deleted_id || (i+1) as i32 == deleted_id {i+2} else {i+1};
        
        if deleted_id == vect.len() as i32 - 1 && next_id >= vect.len() - 1 {return true;}

        if desc && (vect[actual_id] <= vect[next_id] || (vect[actual_id] - vect[next_id]).abs() > 3){
            if deleted_id == vect.len() as i32 - 1{
                return false;
            }
            return is_safe(vect, desc, deleted_id+1);     
        }
        else if !desc && (vect[actual_id] >= vect[next_id] || (vect[actual_id] - vect[next_id]).abs() > 3){
            if deleted_id == vect.len() as i32 - 1{
                return false;
            }
            return is_safe(vect, desc, deleted_id+1);
        }
        else if next_id >= vect.len() - 1 {return true;}
    }
    return false;
}


#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day2\\input_day2.txt"){
        Ok(content) => {
            let mut total_safes: u32 = 0;

            for line in content.lines(){
                let vect: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
                let desc:bool = is_desc(&vect);
                
                if is_safe(&vect,desc, -1) {
                    total_safes+=1;
                }         
            }

            println!("Day 2, star 2: {}",total_safes) // < a 743
        }
        Err(_e) => {
            print!("Error detected {}",_e);
        }
    }
}