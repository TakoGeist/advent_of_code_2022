
fn signal_strength(input: &str) -> isize{
    let mut cycle_count = 1;
    let mut strength = 1;
    let mut res = 0;
    let mut mult = 20;
    for line in input.lines(){
        match line.starts_with("a") {
            true  => {cycle_count+=2; strength += line.split_once(" ").unwrap().1.parse::<isize>().unwrap()},
            false => cycle_count += 1,
        }
        if cycle_count == mult || cycle_count == mult-1{
            res += mult * strength;
            mult += 40;
        }
    }
    res
}

fn draw_to_screen(input: &str) -> String{
    let mut screen = String::new();
    let mut position: i32 = 1;
    let mut cycle: i32 = 0;
    for line in input.lines(){
        if cycle == 40{
            screen += "\r\n";
            cycle -= 40;
        }
        screen += (cycle.abs_diff(position) > 1).then(|| ".").or(Some("#")).unwrap();
        cycle += 1;
        if cycle == 40{
            screen += "\n";
            cycle -= 40;
        }
        match line.starts_with("a"){
            true => {screen += (cycle.abs_diff(position) > 1).then(|| ".").or(Some("#")).unwrap();
                     cycle += 1; position += line.split_once(" ").unwrap().1.parse::<i32>().unwrap();},
            false => (),
        }
    }
    screen
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Part 1 -> {}",signal_strength(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("vvv Part 2 vvv \n{}",draw_to_screen(&input));
} 
