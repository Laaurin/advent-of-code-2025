use utils;

fn main() {
    let input = utils::read_input("inputs/day02.txt");
    println!("part 1: {}", solve_part1(&input))
}

fn solve_part1(input:&str) -> i128 {
    let mut sum:i128 = 0;
    let ranges = input.trim().split(',');
    for range in ranges {
        if let Some((start_str, end_str)) = range.split_once('-') {
            let start:i128 = start_str.parse::<i128>().unwrap();
            let end:i128 = end_str.parse::<i128>().unwrap();
            println!("interval {} - {}", start, end);
            for i in start..=end {
                let number_str = i.to_string();

                if check_id_valid(&number_str) {
                    sum += i;
                }
            }
        }
        
    }
    sum
}

fn check_id_valid(id :&str) -> bool{
    let chars : Vec<char> = id.chars().collect();
    let len = chars.len();
    if len%2 != 0 {return false;}
    let half = len/2;
    for i in 0..half {
        if chars[i] != chars[half+i] {return false;}
    }
    return true;
}
