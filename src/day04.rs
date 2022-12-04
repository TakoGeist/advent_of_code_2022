
fn contained(input: &str) -> usize{
    input.lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a,b)| 
            (a.split_once("-").unwrap(),b.split_once("-").unwrap()))
        .map(|((a,b),(c,d))|
            ((a.parse::<i32>().unwrap(),b.parse::<i32>().unwrap()),(c.parse::<i32>().unwrap(),d.parse::<i32>().unwrap())))
        .filter(|((a,b),(c,d))| (a <= c && b >= d) || c <= a && d >= b)
        .count()
}
    
fn overlap(input: &str) -> usize{
    input.lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a,b)| 
            (a.split_once("-").unwrap(),b.split_once("-").unwrap()))
        .map(|((a,b),(c,d))|
            ((a.parse::<i32>().unwrap(),b.parse::<i32>().unwrap()),(c.parse::<i32>().unwrap(),d.parse::<i32>().unwrap())))
        .filter(|((a,b),(c,d))| !(d < a || b < c))
        .count()    
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day04.txt").unwrap();
    println!("Part 1 -> {}",contained(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day04.txt").unwrap();
    println!("Part 2 -> {}",overlap(&input));
} 
