
fn round(op: char, me: char) -> u32{
    match (op,me){
        ('A','X') => 3 + 1,
        ('A','Y') => 6 + 2,
        ('A','Z') => 0 + 3,
        ('B','X') => 0 + 1,
        ('B','Y') => 3 + 2,
        ('B','Z') => 6 + 3,
        ('C','X') => 6 + 1,
        ('C','Y') => 0 + 2,
        ('C','Z') => 3 + 3,
        _ => 0
    }
}

fn rigged_round(op: char, me: char) -> u32{
    match (op,me){
        ('A','X') => 0 + 3,
        ('A','Y') => 3 + 1,
        ('A','Z') => 6 + 2,
        ('B','X') => 0 + 1,
        ('B','Y') => 3 + 2,
        ('B','Z') => 6 + 3,
        ('C','X') => 0 + 2,
        ('C','Y') => 3 + 3,
        ('C','Z') => 6 + 1,
        _ => 0
    }
}

fn game_rps(input: &str) -> u32{
    input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold(0 , |acc,(op,me)| 
            acc + round(op.chars().nth(0).unwrap(), me.chars().nth(0).unwrap())
        )
}

fn rigged_game_rps(input: &str) -> u32{
    input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold(0 , |acc,(op,me)| 
            acc + rigged_round(op.chars().nth(0).unwrap(), me.chars().nth(0).unwrap())
    )
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    println!("Part 1 -> {}", game_rps(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    println!("Part 2 -> {}", rigged_game_rps(&input));
} 
