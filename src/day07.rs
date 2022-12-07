
fn dir_sum(input: &mut std::str::Lines, total_sum: &mut usize) -> usize{
    let mut sum = 0;
    while let Some(x) = input.next(){
        match x{
            "$ cd .." => break,
            _ if x.starts_with("$ ls") => (),
            _ if x.starts_with("dir") => (),
            _ if x.starts_with("$") => sum += dir_sum(input, total_sum),
            _ => sum += x.split_once(" ").unwrap().0.parse::<usize>().unwrap(),
        }
    }
    if sum < 100000{
        *total_sum += sum;
    }
    sum
}

fn find_dir(input: &str) -> usize{
    let mut total_sum: usize = 0;
    let mut lines = input.lines().into_iter();
    dir_sum(&mut lines, &mut total_sum);
    total_sum
}

fn dir_sizes(input: &mut std::str::Lines, sum_list: &mut Vec<usize>) -> usize{
    let mut sum = 0;
    while let Some(x) = input.next(){
        match x{
            "$ cd .." => break,
            _ if x.starts_with("$ cd") => sum += dir_sizes(input, sum_list),
            _ if x.starts_with("$ ls") => (),
            _ if x.starts_with("dir") => (),
            _ => sum += x.split_once(" ").unwrap().0.parse::<usize>().unwrap(),
        }
    }
    sum_list.push(sum);
    sum
}

fn free_space(input: &str) -> usize{
    let mut total_sum = Vec::new();
    let mut lines = input.lines().into_iter();
    dir_sizes(&mut lines, &mut total_sum);
    total_sum.sort_unstable();
    *total_sum.iter().filter(|x| **x >= total_sum[total_sum.len()-1] - 40000000).min().unwrap()
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day07.txt").unwrap();
    println!("Part 1 -> {}", find_dir(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day07.txt").unwrap();
    println!("Part 2 -> {}", free_space(&input));
} 
