use crate::utils;

#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day3\\input_day3.txt"){
        Ok(_content) => {
            ();
        }
        Err(_e) => {
            print!("Error detected: {}",_e);
        }
    }
}