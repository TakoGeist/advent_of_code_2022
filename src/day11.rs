
enum OP{
    Square,
    Mul(usize),
    Add(usize),
}

impl OP{
    fn from_str(input: &str) -> OP{
        if let Some(x) = input.strip_prefix("new = old * "){
            if let Ok(num) = x.parse::<usize>(){
                return OP::Mul(num);
            }
            else{
                return OP::Square;
            }
        }
        else if let Some(num) = input.strip_prefix("new = old + "){
            return OP::Add(num.parse::<usize>().unwrap())
        }   
        panic!()
    }

    fn update(&self, item: usize) -> usize{
        match self{
            OP::Add(x) => item + *x,
            OP::Mul(x) => item * *x,
            OP::Square => item * item,
        }
    }
}

struct MONKEY{
    items: Vec<usize>,
    update: OP,
    test: usize,
    tr: usize,
    fl: usize,
    inspected: usize
}

impl MONKEY{
    fn from_str(input: &str) -> MONKEY{
        let mut lines = input.lines().skip(1);
        MONKEY { items: lines.next().unwrap().strip_prefix("  Starting items:").unwrap().split(",")
                    .map(|num| num.trim().parse::<usize>().unwrap()).collect::<Vec<_>>(), 
            update: OP::from_str(lines.next().unwrap().strip_prefix("  Operation: ").unwrap()),
            test: lines.next().unwrap().split(" ").map(|x| x.parse::<usize>()).flatten().nth(0).unwrap(), 
            tr: lines.next().unwrap().chars().filter(|x| x.is_numeric()).collect::<String>().parse::<usize>().unwrap(), 
            fl: lines.next().unwrap().chars().filter(|x| x.is_numeric()).collect::<String>().parse::<usize>().unwrap(),
            inspected: 0
        }
    }
}

fn monkey_business<F>(monkeys: &mut Vec<MONKEY>, rounds: u32, mut relief: F) -> usize
    where F: FnMut(usize) -> usize{

    for _ in 0..rounds{
        for ind_mo in 0..monkeys.len(){
            monkeys[ind_mo].inspected += monkeys[ind_mo].items.len();
            while let Some(item) = monkeys[ind_mo].items.pop(){
                let new = relief(OP::update(&monkeys[ind_mo].update, item));
                if new % monkeys[ind_mo].test == 0{
                    let new_monkey = monkeys[ind_mo].tr;
                    monkeys[new_monkey].items.push(new);
                }
                else{
                    let new_monkey = monkeys[ind_mo].fl;
                    monkeys[new_monkey].items.push(new);
                }
            }
        }
    }

    monkeys.sort_by(|a,b| b.inspected.cmp(&a.inspected));
    monkeys.iter().take(2).map(|x| x.inspected).product()

}

fn monkey_20(input: &str) -> usize{
    let mut monkeys = input.split("\r\n\r\n").map(|block| MONKEY::from_str(block)).collect::<Vec<_>>();
    monkey_business(&mut monkeys, 20, |x| x / 3)
}

fn monkey_10000(input: &str) -> usize{
    let mut monkeys = input.split("\r\n\r\n").map(|block| MONKEY::from_str(block)).collect::<Vec<_>>();
    let relief = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test);
    monkey_business(&mut monkeys, 10000, |x| x % relief)
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("Part 1 -> {}", monkey_20(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("Part 1 -> {}", monkey_10000(&input));
} 
