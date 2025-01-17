// use std::array;
// use std::env;
use std::fs;

const FILE_PATH:&str = "input.txt";

fn main() {
    
    // println!("In file {FILE_PATH}");

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    // let mut list_a: [u32; 1001] = [0;1001];
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    // let mut list_b:[u32; 1001] = [0;1001];
    parse_file(contents, &mut list_a, &mut list_b);
    // println!("list_a:\n{:?}", list_a);
    // println!("list_b:\n{:?}", list_b);

    let result = calc_similarity(&list_a, &list_b);
    println!("sum:\n{:?}", result);
    let result = calc_distance(&mut list_a, &mut list_b);
    println!("sum:\n{:?}", result);
}

fn remove_and_return_smallest(numbers: &mut Vec<i32>) -> Option<(i32, Vec<i32>)> {
    if numbers.is_empty() {
        return None; // Return None if the input is empty
    }

    let min_index = numbers
        .iter()
        .enumerate()
        .min_by_key(|&(_, &val)| val)
        .map(|(idx, _)| idx)?;

    let smallest = numbers.remove(min_index);
    Some((smallest, numbers.clone()))
}
fn calc_similarity(list_a:&Vec<i32>, list_b:&Vec<i32>) -> usize{
    let mut sum: usize=0;
    for i in list_a
    {
        println!("looking for {}, {} results found", i, list_b.iter().filter(|&n| *n == *i).count());
        sum += usize::try_from(*i).unwrap() * list_b.iter().filter(|&n| *n == *i).count();
    }
    return sum;
}

fn calc_distance(mut list_a:&mut Vec<i32>, mut list_b:&mut Vec<i32>) -> u32
{
    let mut sum =0;
    
    for _i in 0..list_a.len()
    {
        sum += remove_and_return_smallest(&mut list_a).unwrap().0.abs_diff(remove_and_return_smallest(&mut list_b).unwrap().0);
    }
    return sum;
}


fn parse_file(file_data:String, out_list_a:&mut Vec<i32>, out_list_b:&mut Vec<i32>)
{
    for line in file_data.lines(){
        let mut words = line.split_whitespace();
        out_list_a.push(words.next().unwrap().parse().unwrap());
        out_list_b.push(words.next().unwrap().parse().unwrap());
    }
}
