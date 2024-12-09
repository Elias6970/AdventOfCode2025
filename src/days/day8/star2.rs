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

                        let (mut bigger,mut lower) = if is_first_bigger(positions[i], positions[j]) {
                            (positions[i], positions[j])
                        } else {
                            (positions[j], positions[i])
                        };
                        let distance:(i32,i32) = get_distance(bigger, lower);

                        //Because the antennas now count
                        antinodes.insert(bigger);
                        antinodes.insert(lower);

                        bigger = (bigger.0 + distance.0, bigger.1 + distance.1);
                        while bigger.0 >= 0 && bigger.1 >= 0 && bigger.0 < row_count && bigger.1 < col_count{
                            antinodes.insert(bigger);
                            bigger = (bigger.0 + distance.0, bigger.1 + distance.1);
                        }
                        
                        lower = (lower.0 - distance.0,lower.1 - distance.1);
                        while lower.0 >= 0 && lower.1 >= 0 && lower.0 < row_count && lower.1 < col_count{
                            antinodes.insert(lower);
                            lower = (lower.0 - distance.0,lower.1 - distance.1);
                        }

                    }
                }
            }
            println!("Day 8, star 2:{}",antinodes.len());
            
        }

        Err(e) =>{
            println!("Error detected: {}",e);
        }
    }
}