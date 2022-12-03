use std::collections::HashSet;


fn priorities(input: &str) -> u32{
    let mut res = 0;
    for line in input.lines(){
        let (fst,snd) = line.split_at(line.len()/2); 
        let set = snd.chars().collect::<HashSet<_>>();
        for elem in fst.chars(){
            if set.contains(&elem){
                if ('a'..='z').contains(&elem){
                    res += elem as u32 - b'a' as u32 + 1;
                }
                else{
                    res += elem as u32 - b'A' as u32 + 27;
                }
                break;
            }
        }
    }
    res
}

fn group_priorities(input: &str) -> u32{
    let mut res = 0;
    let mut lines = input.lines();
    for _ in (0..input.lines().count()).step_by(3){
        let line_a = lines.next().unwrap();
        let line_b = lines.next().unwrap().chars().collect::<HashSet<_>>();
        let line_c = lines.next().unwrap().chars().collect::<HashSet<_>>();
        for elem in line_a.chars(){
            if line_b.contains(&elem) && line_c.contains(&elem){
                if ('a'..='z').contains(&elem){
                    res += elem as u32 - b'a' as u32 + 1;
                }
                else{
                    res += elem as u32 - b'A' as u32 + 27;
                }
                break;
            }
        }
    }
    res
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day03.txt").unwrap();
    println!("Part 1 -> {}",priorities(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day03.txt").unwrap();
    println!("Part 2 -> {}",group_priorities(&input));
} 
