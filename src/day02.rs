
fn round(op: u8, me: u8) -> u8{
    match (op,me - 23){
        (b'A',b'A') => 3,
        (b'A',b'B') => 6,
        (b'A',b'C') => 0,
        (b'B',b'A') => 0,
        (b'B',b'B') => 3,
        (b'B',b'C') => 6,
        (b'C',b'A') => 6,
        (b'C',b'B') => 0,
        (b'C',b'C') => 3,
        _ => 0
    }
}

fn rigged_round(op: u8, me: u8) -> u32{
    match (op,me - 23){
        (b'A',b'A') => 0 + 3,
        (b'A',b'B') => 3 + 1,
        (b'A',b'C') => 6 + 2,
        (b'B',b'A') => 0 + 1,
        (b'B',b'B') => 3 + 2,
        (b'B',b'C') => 6 + 3,
        (b'C',b'A') => 0 + 2,
        (b'C',b'B') => 3 + 3,
        (b'C',b'C') => 6 + 1,
        _ => 0
    }
}

fn game_rps(input: &str) -> u32{
    input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold(0 , |acc,(op,me)| {
            let oponent = op.trim().chars().next().unwrap() as u8;
            let play  = me.trim().chars().next().unwrap() as u8;
            acc + (round(oponent, play) + play - b'W') as u32
        })
}

fn rigged_game_rps(input: &str) -> u32{
    input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold(0 , |acc,(op,me)| {
            let oponent = op.trim().chars().next().unwrap() as u8;
            let play  = me.trim().chars().next().unwrap() as u8;
            acc + rigged_round(oponent, play)
        })
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    println!("Part 1 -> {}", game_rps(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    println!("Part 2 -> {}", rigged_game_rps(&input));
} 
