use std::fs;

const FILE_PATH:&str = "input.txt";
fn main() {
    let contents = fs::read_to_string(FILE_PATH)
    .expect("Should have been able to read the file");

    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    // let mut list_b:[u32; 1001] = [0;1001];
    parse_file(contents, &mut rules ,&mut updates);
    
    // println!("rules:\n{:?}", rules);
    // println!("updates:\n{:?}", updates);
    println!("total updates: {}", updates.len());

    // let valid_updates = _count_valid_updates(&mut rules ,&mut updates);
    // println!("valid updates: {}", valid_updates);
    
    // let invalid_updates= remove_invalid_updates(&mut rules ,&mut updates);
    // println!("invalid updates: {}", invalid_updates);
    
    // let sum = sum_middle_pages(updates);
    // println!("sum: {}", sum);
    
    // part two
    
    let valid_updates= remove_valid_updates(&mut rules ,&mut updates);
    println!("removed valid updates");
    println!("valid updates: {}", valid_updates);
    
    reorder_invalid_updates(&rules ,&mut updates);

    let sum = sum_middle_pages(updates);
    println!("sum: {}", sum);

}

fn reorder_invalid_updates(rules: &Vec<Vec<i32>>, updates:&mut Vec<Vec<i32>>)
{
    let mut i = 0 as usize;
    while  i < updates.len()
    {
        fix_update
        let mut swap_index = 0;
        while !is_update_valid(rules, &updates[i])
        {
            updates.swap(i);
            swap_index+=1;
            continue;
        }
        i+=1;
    }
}

fn sum_middle_pages(updates:Vec<Vec<i32>>) -> i32
{
    let mut middle_page_sum = 0;
    for update in updates{
        middle_page_sum+= update[update.len()/2];
    }
    middle_page_sum
}

fn is_update_passed_rule(rule: &Vec<i32>, update:&Vec<i32>) -> bool
{
    let first_index =  match update.iter().position(|&r| r == rule[0])
    {
        None => return true,
        Some(x) => x

    };
    let second_index =   match update.iter().position(|&r| r == rule[1])
    {
        None => return true,
        Some(x) => x

    };
    return first_index < second_index;
}

fn is_update_valid(rules: &Vec<Vec<i32>>, update:&Vec<i32>) -> bool
{
    for rule in rules{
        if !is_update_passed_rule(rule, update)
        {
            return false;
        }
    }
    return true;
}

fn remove_invalid_updates(rules: &mut Vec<Vec<i32>>, updates:&mut Vec<Vec<i32>>) -> i32
{
    let mut i = 0 as usize;
    let mut invalid_counter = 0;
    while  i < updates.len()
    {
        if !is_update_valid(rules, &updates[i])
        {
            updates.remove(i);
            invalid_counter+=1;
            continue;
        }
        i+=1;
    }
    invalid_counter
}

fn remove_valid_updates(rules: &mut Vec<Vec<i32>>, updates:&mut Vec<Vec<i32>>) -> i32
{
    let mut i = 0 as usize;
    let mut valid_counter = 0;
    while  i < updates.len()
    {
        if is_update_valid(rules, &updates[i])
        {
            updates.remove(i);
            valid_counter+=1;
            continue;
        }
        i+=1;
    }
    valid_counter
}

fn _count_valid_updates(rules: &Vec<Vec<i32>>, updates:&Vec<Vec<i32>>) -> i32
{
    let mut valid_updates = 0;
    for update in updates
    {
        if is_update_valid(rules, update)
        {
            valid_updates +=1;
        }
    }
    return valid_updates;
}

fn split_to_int_vector(input: &str) -> Vec<i32> {
    input
    .split(&[',', '|'][..]) // Split by whitespace
    .filter_map(|s| s.parse::<i32>().ok()) // Parse each part to i32, ignoring invalid parts
    .collect() // Collect into a Vec<i32>
}

    
fn parse_file(file_data:String, rules: &mut Vec<Vec<i32>>, updates:&mut Vec<Vec<i32>>)
{
    let mut is_rules_section = true;
    for line in file_data.lines(){
        if line == ""{
            is_rules_section=false;
            continue;
        }
        let row: Vec<i32> = split_to_int_vector(line);
        if is_rules_section
        {
            rules.push(row);
        }
        else
        {
            updates.push(row);
        }
    }
}

