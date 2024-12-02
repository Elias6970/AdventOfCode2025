use crate::utils;

#[allow(dead_code)]
pub fn solve() {
    match utils::read_file_content("src\\days\\day1\\input_star1.txt"){
        Ok(content) => {
            let num_lines: usize = content.lines().count();
            let mut first_column: Vec<i32> = vec![0;num_lines];
            let mut second_column: Vec<i32> = vec![0;num_lines];

            let mut i: usize = 0;
            for line in content.lines(){
                if let Some((part1,part2)) = line.split_once("   "){
                    first_column[i] = part1.parse().unwrap_or(0);
                    second_column[i] = part2.parse().unwrap_or(0);
                }
                i+=1;
            }
            first_column.sort();
            second_column.sort();

            let mut total_distance: i32 = 0;

            for i in 0..num_lines{
                total_distance += (first_column[i] - second_column[i]).abs();
            }

            println!("Result: {}",total_distance);
        }
        Err(_e) => {
            print!("Error detected {}",_e);
        }
    }
}