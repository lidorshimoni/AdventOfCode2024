use std::{fs, usize};

const FILE_PATH:&str = "input.txt";
// const FILE_PATH:&str = "test.txt";


fn main() {
    let contents = fs::read_to_string(FILE_PATH)
    .expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<char>> = Vec::new();

    // let mut list_b:[u32; 1001] = [0;1001];
    parse_file(contents, &mut matrix);

    // println!("safe reports = {}", count_safe_reports(reports.clone()));
    // println!("safe reports 2 = {}", count_safe_reports2(reports));
    // println!("mat:\n{:?}", matrix);
    println!("XMAS found {} times!", count_word(&mut matrix.clone(), "XMAS"));
    println!("X MAS found {} times!", count_x_word(&mut matrix.clone(), "MAS"));
    println!("X MAS found {} times!", count_x_mas(&mut matrix.clone()));
    // println!("SAMX found {} times!", count_word(&mut matrix.clone(), "SAMX"));
    // println!("{}", is_report_safe(Vec::from([8,6,4,4,1])));
}

fn count_x_mas(grid: &Vec<Vec<char>>) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut count = 0;

    for row in 1..m - 1 {
        for col in 1..n - 1 {
            // Check MAS pattern in "X" shape
            if (grid[row][col] == 'A') && 
                  ((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S') ||
                    (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M'))
                &&
               (grid[row + 1][col - 1] == 'S' && grid[row - 1][col + 1] == 'M' ||
                grid[row + 1][col - 1] == 'M' && grid[row - 1][col + 1] == 'S') {
                count += 1;
            }
        }
    }

    count
}
fn count_x_word(mat:&mut Vec<Vec<char>>, word: &str) -> i32
{
    let mut counter=0;
    for row in 0..mat.len()
    {
        for col in 0..mat[row].len()
        {
            counter+=count_x_word_at_index(mat, row,col, word) as i32;
        }
    }
    return counter;
}

fn count_x_word_at_index(grid: &mut Vec<Vec<char>>, row: usize, col: usize, word: &str) -> u32 {

    let word_chars: Vec<char> = word.chars().collect();
    let mut found_count=0;

    // Return false if the given coordinate does not match with the first character of the word
    if grid[row][col] != word_chars[0] {
        return 0;
    }

    // Directions for 8 possible moves: (x, y)
    let directions = [
        (-1, -1), (-1, 1),
        (1, -1), (1, 1),
    ];
    let reversed_word = reverse_string(word);

    // Search in all 8 directions
    for &(dx, dy) in &directions {
        
        if is_word_at_position_and_direction(grid, row, col, dx, dy, word) 
        || is_word_at_position_and_direction(grid, row, col, dx, dy, &reversed_word)
        {
            let new_row = row as isize+(dx*(word.len()-1) as isize);
            // let new_col = col as isize+(dy*(word.len()-1) as isize);

            if new_row <0 {return 0};
            if is_word_at_position_and_direction(grid, new_row as usize, col, dx*-1, dy, word)
             || is_word_at_position_and_direction(grid, new_row as usize, col, dx*-1, dy, &reversed_word)
            //  || is_word_at_position_and_direction(grid, row, new_col as usize, dx, dy*-1, &word)
            //  || is_word_at_position_and_direction(grid, row, new_col as usize, dx, dy*-1, &reversed_word)
            {
                found_count+=1;
                // grid[row][col] = '0';
            }
        }
    }

    // If the word is not found in any direction, return false
    return found_count;
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn is_word_at_position_and_direction(grid: &Vec<Vec<char>>, row: usize, col: usize, dx: isize, dy: isize, word: &str) -> bool
{
    let mut curr_x = row as isize + dx;
    let mut curr_y = col as isize + dy;
    let mut k = 1;
    let m = grid.len();
    let n = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();


    while k < word.len() {
        // Break if out of bounds
        if curr_x < 0 || curr_y < 0 || curr_x >= m as isize || curr_y >= n as isize {
            break;
        }

        // Break if characters don't match
        if grid[curr_x as usize][curr_y as usize] != word_chars[k] {
            break;
        }

        // Move in the current direction
        curr_x += dx;
        curr_y += dy;
        k += 1;
    }

    // If all characters matched, return true
    return k == word.len()
}

fn count_word(mat:&mut Vec<Vec<char>>, word: &str) -> i32
{
    let mut counter=0;
    for row in 0..mat.len()
    {
        for col in 0..mat[row].len()
        {
            counter+=count_word_at_index(mat, row,col, word) as i32;
        }
    }
    return counter;
}

fn count_word_at_index(grid: &Vec<Vec<char>>, row: usize, col: usize, word: &str) -> u32 {
    let word_chars: Vec<char> = word.chars().collect();
    let mut found_count=0;

    // Return false if the given coordinate does not match with the first character of the word
    if grid[row][col] != word_chars[0] {
        return 0;
    }

    // Directions for 8 possible moves: (x, y)
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    // Search in all 8 directions
    for &(dx, dy) in &directions {
        if is_word_at_position_and_direction(grid, row, col, dx, dy, word)
        {
            found_count+=1;
        }
    }

    // If the word is not found in any direction, return false
    return found_count;
}

fn _print_puzzle(mat:&Vec<Vec<char>>)
{
    for (_row_index,row) in mat.iter().enumerate()
    {
        for (_col_index,cell) in row.iter().enumerate()
        {
            print!("{} ",cell);
        }
        println!("");
    }
}

fn parse_file(file_data:String, mat:&mut Vec<Vec<char>>)
{
    for line in file_data.lines(){
        let row: Vec<char> = line.chars().collect();
        mat.push(row);
    }
}
