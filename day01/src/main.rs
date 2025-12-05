use utils;

fn main() {

    let input = utils::read_input("inputs/day01.txt");
    println!("Teil 1: {}", solve_part2(&input));
    
}

fn solve_part2(input: &str) -> i32 {
    //print!("{}", input);
    let rotation_instructions = input.split("\n");
    let mut dial = 50;
    let mut result = 0;
    for insturction in rotation_instructions {

        let mut chars = insturction.chars();
        if let Some(direction) = chars.next() {
            let number : i32 = chars.as_str().parse().unwrap();
            let (new_dial, hits) = match direction {
                'R' => calc23(dial, number),
                'L' => calc23(dial, -number),
                _ => (dial, 0),
            };
            dial = new_dial;
            result += hits;
            //if dial == 0 {result += 1;}

        }
    }
    result
}

fn calc2old(a: i32, b: i32) -> (i32, i32) {
    let mut num = a+b;
    let mut hit = 0;
    loop {
        if num > 99 {
            num -= 100;
            hit += 1;
        }
        else if num < 0 {
            num += 100;
            hit += 1;
        }
        else if num == 0 {
        //    hit += 1;
        }
        if 0 <= num && num < 100 {
            return (num, hit);
        }
    }
}
//6932

fn calc2(current_dial: i32, steps: i32) -> (i32, i32) {
    let mut dial = current_dial;
    let mut hits = 0;
    
    // Wir bestimmen die Richtung (+1 oder -1)
    let direction = if steps > 0 { 1 } else { -1 };
    
    // Wir führen jeden einzelnen "Klick" aus
    for _ in 0..steps.abs() {
        dial += direction;
        
        // Wrap-Around behandeln
        if dial == 100 { dial = 0; }
        if dial == -1  { dial = 99; }
        
        // Jetzt prüfen wir ganz einfach: Sind wir auf 0?
        if dial == 0 {
            hits += 1;
        }
    }
    
    (dial, hits)
}

fn solve_part1(input: &str) -> i32 {
    //print!("{}", input);
    let rotation_instructions = input.split("\n");
    let mut dial = 50;
    let mut result = 0;
    for insturction in rotation_instructions {

        let mut chars = insturction.chars();
        if let Some(direction) = chars.next() {
            let number : i32 = chars.as_str().parse().unwrap();
            match direction {
                'R' => dial = calc(dial, number),
                'L' => dial = calc(dial, -number),
                _ => println!("{}", "nichts"),
            }
            println!("{}", dial);
            if dial == 0 {
                result += 1;
            }
        }
    }
    result
}

fn calc(a: i32, b: i32) -> i32 {
    let mut num = a+b;
    loop {
        if num > 99 {
            num -= 100;
        }
        if num < 0 {
            num += 100;
        }
        if 0 <= num && num < 100 {
            return num;
        }
    }
}
