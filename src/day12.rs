use std::collections::VecDeque;

fn bfs(map: &Vec<Vec<i8>>, start: (isize,isize), end: (isize,isize)) -> Option<usize>{
    let neighbours = [(0,-1),(-1,0),(1,0),(0,1)];
    let bounds = (0..map.len() as isize,0..map[0].len() as isize);
    let mut frontier = VecDeque::from([(start,0)]);
    let mut visited = vec![vec![false;map[0].len()];map.len()];
    visited[start.0 as usize][start.1 as usize] = true;
    let (mut current, mut distance);
    while !frontier.is_empty(){
        (current, distance) = frontier.pop_front().unwrap();
        if current == end{
            return Some(distance);
        }
        for elem in neighbours.iter().map(|(a,b)| (current.0+a,current.1+b)){
            if bounds.0.contains(&elem.0) && bounds.1.contains(&elem.1){
                if map[elem.0 as usize][elem.1 as usize] <= map[current.0 as usize][current.1 as usize] + 1
                        && !visited[elem.0 as usize][elem.1 as usize]{
                    visited[elem.0 as usize][elem.1 as usize] = true;
                    frontier.push_back((elem, distance + 1));
                }
            }
        }
    }
    None
}

fn fewest_steps(input: &str) -> usize{
    let mut start: (isize,isize) = (0,0);
    let mut end: (isize,isize) = (0,0);
    let map = input.lines().enumerate()
        .fold(Vec::new(), |mut acc0,(row,line)| {
            acc0.push(line.chars().enumerate()
            .fold(Vec::new(), |mut acc1, (col,elem)|{
                match elem{
                    'S' => {start = (row as isize,col as isize); acc1.push(0);},
                    'E' => {end = (row as isize,col as isize); acc1.push(25);},
                    _ => acc1.push(elem as i8 - b'a' as i8),
                }
                acc1
            }));
            acc0
        });

    bfs(&map, start, end).unwrap()
}

fn best_start(input: &str) -> usize{
    let mut start: Vec<(isize,isize)> = Vec::new();
    let mut end: (isize,isize) = (0,0);
    let map = input.lines().enumerate()
        .fold(Vec::new(), |mut acc0,(row,line)| {
            acc0.push(line.chars().enumerate()
            .fold(Vec::new(), |mut acc1, (col,elem)|{
                match elem{
                    'S' | 'a' => {start.push((row as isize,col as isize)); acc1.push(0);},
                    'E' => {end = (row as isize,col as isize); acc1.push(25);},
                    _ => acc1.push(elem as i8 - b'a' as i8),
                }
                acc1
            }));
            acc0
        });
        
    start.iter().map(|x| bfs(&map, *x, end)).flatten().min().unwrap()
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("Part 1 -> {}", fewest_steps(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("Part 2 -> {}", best_start(&input));
} 
