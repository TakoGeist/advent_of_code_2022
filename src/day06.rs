
fn start_of_packet(input: &str) -> usize{
    let input = input.as_bytes();
    'outer: for pos in 4..input.len(){
        let mut visited = [false;26];
        for ind in pos-4..pos{
            let visit = &mut visited[(input[ind]-b'a') as usize]; 
            if *visit{
                continue 'outer;
            }
            *visit = true;
        }
        return pos;
    }
    0
}

fn message(input: &str) -> usize{
    let input = input.as_bytes();
    'outer: for pos in 14..input.len(){
        let mut visited = [false;26];
        for ind in pos-14..pos{
            let visit = &mut visited[(input[ind]-b'a') as usize]; 
            if *visit{
                continue 'outer;
            }
            *visit = true;
        }
        return pos;
    }
    0
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day06.txt").unwrap();
    println!("Part 1 -> {}",start_of_packet(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day06.txt").unwrap();
    println!("Part 2 -> {}",message(&input));
} 
