use std::collections::HashSet;

fn simulate(input: &str, length: usize) -> usize{

    let mut visits: HashSet<(i32,i32)> = HashSet::from([(0,0)]);

    let mut rope: Vec<(i32, i32)> = vec![(0,0);length];
    let mut mov: (i32,i32);
    let mut num: i32;
    for line in input.lines(){
        match line.split_once(" ").unwrap(){
            ("U", x) => {mov = (0,1); num = x.parse::<i32>().unwrap();},
            ("D", x) => {mov = (0,-1); num = x.parse::<i32>().unwrap();},
            ("R", x) => {mov = (1,0); num = x.parse::<i32>().unwrap();},
            ("L", x) => {mov = (-1,0); num = x.parse::<i32>().unwrap();},
            _ => panic!(),
        }

        for _ in 0..num{
            rope[0].0 += mov.0;
            rope[0].1 += mov.1;
            for i in 1..length{
                let x = rope[i-1].0 - rope[i].0;
                let y = rope[i-1].1 - rope[i].1;
                if x.abs() > 1 && y == 0{
                    rope[i].0 += (x < 0).then(|| -1).or(Some(1)).unwrap();
                }else if x == 0 && y.abs() > 1{
                    rope[i].1 += (y < 0).then(|| -1).or(Some(1)).unwrap();
                }else if x.abs() > 1 || y.abs() > 1{
                    rope[i].0 += (x < 0).then(|| -1).or(Some(1)).unwrap();
                    rope[i].1 += (y < 0).then(|| -1).or(Some(1)).unwrap();
                }
            }
            visits.insert(rope[length-1]);
        }
    }

    visits.len()
}

fn visited(input: &str) -> usize{
    simulate(input, 2)
}

fn tail_visits(input: &str) -> usize{
    simulate(input, 10)
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day09.txt").unwrap();
    println!("Part 1 -> {}",visited(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day09.txt").unwrap();
    println!("Part 2 -> {}", tail_visits(&input));
} 
