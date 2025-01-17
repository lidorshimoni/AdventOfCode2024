use std::fs;

const FILE_PATH:&str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
    .expect("Should have been able to read the file");

    let mut reports: Vec<Vec<i32>> = Vec::new();

    // let mut list_b:[u32; 1001] = [0;1001];
    parse_file(contents, &mut reports);
    println!("safe reports = {}", count_safe_reports(reports.clone()));
    println!("safe reports 2 = {}", count_safe_reports2(reports));
    // println!("reports:\n{:?}", reports);
    // println!("{}", is_report_safe(Vec::from([8,6,4,4,1])));
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> i32
{
    let mut num_of_safe_reports = 0;
    for report in reports
    {
        if is_report_safe(report)
        {
            num_of_safe_reports+=1;
        }
    }
    return num_of_safe_reports
}

fn count_safe_reports2(reports: Vec<Vec<i32>>) -> i32
{
    let mut num_of_safe_reports = 0;
    for report in reports
    {
        for i in 0..report.len() {
            let mut temp_report = report.clone();
            temp_report.remove(i);
            if is_report_safe(temp_report)
            {
                num_of_safe_reports+=1;
                break;
            }
            
        }
    }
    return num_of_safe_reports
}

fn is_report_safe(report: Vec<i32>) -> bool
{
    let mut are_levels_rising = None;
    for pair in report.windows(2){
        let diff = pair[0].abs_diff(pair[1]);
        if diff <1 || diff > 3
        {
            return false;
        }
        if are_levels_rising==None
        {
            are_levels_rising = Some((pair[0]-pair[1])>0);
        }
        else if are_levels_rising != (Some((pair[0]-pair[1])>0)) {
            return false;
        }
    }
    return true;
}

fn split_to_int_vector(input: &str) -> Vec<i32> {
    input
        .split_whitespace() // Split by whitespace
        .filter_map(|s| s.parse::<i32>().ok()) // Parse each part to i32, ignoring invalid parts
        .collect() // Collect into a Vec<i32>
}


fn parse_file(file_data:String, out_reports:&mut Vec<Vec<i32>>)
{
    for line in file_data.lines(){
        let levels: Vec<i32> = split_to_int_vector(line);
        out_reports.push(levels);
    }
}
