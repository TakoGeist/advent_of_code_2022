
fn horizontal_check(forest: &Vec<Vec<i8>>, visible: &mut Vec<Vec<bool>>, 
                    hor_range: impl Iterator<Item = usize> + Clone){
    for lin in 0..forest.len(){
        let mut max: i8 = -1;
        for col in hor_range.clone(){
            if max < forest[lin][col]{
                max = forest[lin][col];
                visible[lin][col] = true;
            }
        }
    }
}

fn vertical_check(forest: &Vec<Vec<i8>>, visible: &mut Vec<Vec<bool>>, 
                  ver_range: impl Iterator<Item = usize> + Clone){
    for col in 0..forest[0].len(){
        let mut max: i8 = -1;
        for lin in ver_range.clone(){
            if max < forest[lin][col]{
                max = forest[lin][col];
                visible[lin][col] = true;
            }
        }
    }
}

fn visibility(input: &str) -> usize{
    let forest = input.lines().map(|line| line.chars()
        .map(|x| x.to_digit(10).unwrap() as i8).collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut visible = vec![vec![false;forest[0].len()];forest.len()];
        
        let horizontal = 0..forest[0].len();
        let vertical = 0..forest.len();
        
    horizontal_check(&forest, &mut visible, horizontal.clone());
    horizontal_check(&forest, &mut visible, horizontal.rev());
    vertical_check(&forest, &mut visible, vertical.clone());
    vertical_check(&forest, &mut visible, vertical.rev());
    
    visible.into_iter().map(|line| line.into_iter().filter(|x| *x).count()).sum()
}

fn find_tree(input: &str) -> usize{
    let forest = input.lines().map(|line| line.chars()
        .map(|x| x.to_digit(10).unwrap() as i8).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut best_score = 0;

    for lin in 1..forest.len()-1{
        for col in 1..forest[0].len()-1{
            let value = forest[lin][col];
            
            let score: usize = 
                (lin - (0..lin).rev().find(|&x| forest[x][col] >= value).unwrap_or(0))
                * (((lin+1)..forest.len()).find(|&x| forest[x][col] >= value).unwrap_or(forest.len()-1)-lin)
                * (col - (0..col).rev().find(|&x| forest[lin][x] >= value).unwrap_or(0))
                * (((col+1)..forest[0].len()).find(|&x| forest[lin][x] >= value).unwrap_or(forest[0].len()-1)-col);

            if best_score < score{ best_score = score;}
        }
    }
    best_score
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day08.txt").unwrap();
    println!("Part 1 -> {}", visibility(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day08.txt").unwrap();
    println!("Part 2 -> {}", find_tree(&input));
} 
