
fn rearrange(input: &str) -> String{
    let (crates,instructions) = input.split_once("\r\n\r\n").unwrap();
    let mut dock: Vec<Vec<char>> = vec![Vec::new();crates.lines().rev().next().unwrap().chars().filter(|x| x.is_numeric()).count()];
    for line in crates.lines().rev().skip(1){
        for (ind,elem) in line.chars().skip(1).step_by(4).enumerate().filter(|(_,x)| !x.is_whitespace()){
            dock[ind].push(elem);
        }
    }
    let instructions = instructions.lines().map(|line| line.split(" ")
        .map(|word| word.parse::<usize>()).flatten());
    for mut line in instructions{
        let limit = line.next().unwrap();
        let from = line.next().unwrap()-1;
        let to = line.next().unwrap()-1;
        for _ in 0..limit{
            let change = dock[from].pop().unwrap();
            dock[to].push(change);
        }
    } 
    dock.iter().map(|stack| stack.iter().last().unwrap()).collect::<String>()
}

fn rearrange9001(input: &str) -> String{
    let (crates,instructions) = input.split_once("\r\n\r\n").unwrap();
    let mut dock: Vec<Vec<char>> = vec![Vec::new();crates.lines().rev().next().unwrap().chars().filter(|x| x.is_numeric()).count()];
    for line in crates.lines().rev().skip(1){
        for (ind,elem) in line.chars().skip(1).step_by(4).enumerate().filter(|(_,x)| !x.is_whitespace()){
            dock[ind].push(elem);
        }
    }
    let instructions = instructions.lines().map(|line| line.split(" ")
                        .map(|word| word.parse::<usize>()).flatten());
    for mut line in instructions{
        let limit = line.next().unwrap();
        let from = line.next().unwrap()-1;
        let to = line.next().unwrap()-1;
        let mut change:Vec<char> = Vec::new();
        for _ in 0..limit{
            change.push(dock[from].pop().unwrap());
        }
        change.reverse();
        dock[to].append(&mut change);
    } 
    dock.iter().map(|stack| stack.iter().last().unwrap()).collect::<String>()
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day05.txt").unwrap();
    println!("Part 1 ->{}",rearrange(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day05.txt").unwrap();
    println!("Part 2 ->{}",rearrange9001(&input));
} 
