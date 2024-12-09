use crate::utils;

fn find_applying_rules(rules:&Vec<Vec<usize>>,update:&Vec<usize>) -> Vec<Vec<usize>>{
    let mut applying_rules:Vec<Vec<usize>> = Vec::new();
    let mut sort_update: Vec<usize> = update.clone();
    sort_update.sort();

    for i in sort_update{
        let all_indexes:Vec<usize> = rules[0].iter().enumerate().filter_map(|(index,&value)| if value == i { Some(index)} else {None}).collect();
        for j in all_indexes{
            if update.iter().position(|&x| x == rules[1][j]) != None{
                let vect:Vec<usize> = vec![rules[0][j],rules[1][j]];
                applying_rules.push(vect);
            }


        }
    }


    return applying_rules;
}

//Apply the rules and say if the update is in the right order
fn apply_rules(update:&Vec<usize>,applying_rules:&Vec<Vec<usize>>) -> bool{
    //println!("Update: {:?}",&update);
    //println!("Applying rules: {:?}",&applying_rules);
    for i in applying_rules{
        //println!("rule: {:?} -> pos1:{:?},pos2:{:?}",i,update.iter().position(|&x| x == i[0]), update.iter().position(|&x| x == i[1]));
        if update.iter().position(|&x| x == i[0]) >= update.iter().position(|&x| x == i[1]){
            return false;
        }
    }
    return true;
}

fn find_middle_value(arr: &Vec<usize>) -> usize {
    if arr.is_empty() {
        return 0; // Return None if the array is empty
    } else {
        return arr[arr.len() / 2]; // Return the middle element
    }
}
fn sort_update(update:&Vec<usize>,applying_rules:&Vec<Vec<usize>>) -> Vec<usize>{
    let mut new_update: Vec<usize> = update.clone();
    loop{
        for i in 0..applying_rules.len(){
            loop{
                let left:usize = new_update.iter().position(|x| x == &applying_rules[i][0]).unwrap();
                let right:usize = new_update.iter().position(|x| x == &applying_rules[i][1]).unwrap();
                if !(left < right){
                
                    new_update.swap(left, left-1);
                }
                else {
                    break;
                }
            }
        }
        if apply_rules(&new_update, &applying_rules){
            break;
        }
    }
    return new_update;
}

#[allow(dead_code)]
pub fn solve(){
    match utils::read_file_content("src\\days\\day5\\input_day5.txt"){
    //match utils::read_file_content("src\\days\\day5\\test.txt"){
        Ok(content) => {
            let mut rules: Vec<Vec<usize>> = Vec::new(); //Vector for every rule
            //Two vectors with all the rules
            let mut horizontal_rules: Vec<Vec<usize>> = vec![vec![], vec![]];
            //let mut updates: Vec<Vec<usize>> = Vec::new();
            let mut final_result: usize = 0;
            let mut ended_rules: bool = false;

            for i in content.lines(){
                if i == ""{
                    ended_rules = true;
                    
                    //sort rules buy the first number
                    rules.sort();



                    for i in &rules{
                        horizontal_rules[0].push(i[0]);
                        horizontal_rules[1].push(i[1]);
                    }

                    //println!("{:?}",horizontal_rules);
                }

                //Read the rules
                else if !ended_rules{
                    let vec: Vec<usize> = i.split("|").map(|x: &str| x.parse::<usize>().unwrap()).collect();
                    
                    rules.push(vec);
                }
                //Apply the rules
                else {
                    let update: Vec<usize> = i.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                    let applying_rules: Vec<Vec<usize>> = find_applying_rules(&horizontal_rules, &update);
                    
                    if !apply_rules(&update,&applying_rules){
                        let sorted: Vec<usize> = sort_update(&update, &applying_rules);
                        if sorted != Vec::new(){
                            final_result += find_middle_value(&sorted);
                        }
                        
                    }
                }
            }

            println!("Day 5, star 2: {}",final_result);

        }
        Err(_e) => {
            print!("Error detected: {}",_e);
        }
    }
}