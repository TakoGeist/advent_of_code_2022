fn most_calories(input: &str) -> u32{
    input.split("\r\n\r\n")
        .map(|elf| elf.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .sum())
        .max()
        .unwrap()
}
    
fn three_most_calories(input: &str) -> u32{
    let mut sums = 
        input.split("\r\n\r\n")
            .map(|elf| elf.lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum::<u32>())
            .collect::<Vec<_>>();        
    
    sums.sort_unstable_by(|a,b| b.cmp(a));
    
    sums.iter()
        .take(3)
        .sum()    
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day01.txt").unwrap();
    println!("Part 1 -> {}", most_calories(&input))
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day01.txt").unwrap();
    println!("Part 2 -> {}", three_most_calories(&input))    
} 