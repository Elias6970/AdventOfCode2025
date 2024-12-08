use std::collections::HashSet;
use std::collections::HashMap;
use crate::utils;


//Add the two components of the position and say if the first one is bigger
fn is_first_bigger(par1:(i32,i32),par2:(i32,i32)) -> bool{
    return par1.0 + par1.1 > par2.0 + par2.1;
}

//Compute the distance between two points but not abs, 
//only what in going to be added or substracted to the position to get the result points
fn get_distance(bigger:(i32,i32),lower:(i32,i32)) -> (i32,i32){
    return (bigger.0 - lower.0, bigger.1 - lower.1);

}


#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day8\\input_day8.txt"){
    //match utils::read_file_content("src\\days\\day8\\test.txt"){
        Ok(content) => {
            let mut antinodes:HashSet<(i32,i32)> = HashSet::new();
            let mut antennas:HashMap<char,Vec<(i32,i32)>> = HashMap::new();


            let mut row_count:i32 = 0;
            let mut col_count:i32 = 0;
            for line in content.lines(){
                col_count = 0;

                for char in line.chars(){
                    if char != '.' && char != '\n'{
                        if let Some(vec) = antennas.get_mut(&char) {
                            vec.push((row_count,col_count));
                        } else {
                            // If the key doesn't exist, you can insert it with a new Vec
                            antennas.insert(char, vec![(row_count,col_count)]);
                        }
                    }
                    col_count += 1;
                }
                row_count += 1;
            }

            

            //Get the points
            for (_,positions) in &antennas{
                for i in 0..positions.len(){
                    for j in i+1..positions.len(){

                        if is_first_bigger(positions[i], positions[j]){
                            let distance:(i32,i32) = get_distance(positions[i], positions[j]);
                            
                            let mayor:(i32,i32) = (positions[i].0 + distance.0, positions[i].1 + distance.1);
                            let menor:(i32,i32) = (positions[j].0 - distance.0, positions[j].1 - distance.1);

                            if mayor.0 >= 0 && mayor.1 >= 0 && mayor.0 < row_count && mayor.1 < col_count{
                                antinodes.insert(mayor);
                            }
                            if menor.0 >= 0 && menor.1 >= 0 && menor.0 < row_count && menor.1 < col_count{
                                antinodes.insert(menor);
                            }
                        }
                        else{
                            let distance:(i32,i32) = get_distance(positions[j], positions[i]);
                            
                            let mayor:(i32,i32) = (positions[j].0 + distance.0, positions[j].1 + distance.1);
                            let menor:(i32,i32) = (positions[i].0 - distance.0, positions[i].1 - distance.1);

                            if mayor.0 >= 0 && mayor.1 >= 0 && mayor.0 < row_count && mayor.1 < col_count{
                                antinodes.insert(mayor);
                            }
                            if menor.0 >= 0 && menor.1 >= 0 && menor.0 < row_count && menor.1 < col_count{
                                antinodes.insert(menor);
                            }
                        }

                    }
                }
            }

            println!("Day 8, star 1:{}",antinodes.len());
            
        }

        Err(e) =>{
            println!("Error detected: {}",e);
        }
    }
}