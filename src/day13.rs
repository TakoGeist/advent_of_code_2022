
fn correct(left: &[u8], right: &[u8]) -> bool{
    match (left[0], right[0]){
        (a,b) if a==b => correct(&left[1..], &right[1..]), 
        (_, b']') => false,
        (b']', _) => true,
        (b'[', _) => {correct(&left[1..], &[&[right[0],b']'],&right[1..]].concat())}
        (_, b'[') => {correct( &[&[left[0],b']'],&left[1..]].concat(),&right[1..])}
        _ => left[0] < right[0]
    }
    
}

fn index_sum(input: &str) -> usize{
    input.replace("10", ":")
        .split("\r\n\r\n").enumerate().map(|(x,block)| (x, block.split_once("\r\n").unwrap()))
        .filter(|(_,(left, right))| correct(left.as_bytes(), right.as_bytes()))
        .map(|(x,_)| x + 1).sum()
}

fn divider(input: &str) -> usize{
    let res = input.replace("10", ":")
        .lines().filter(|x| !x.is_empty()).map(|x| x.as_bytes())
        .fold((1,2), |mut acc, line| {
            if correct(line, "[[2]]".as_bytes()) {acc.0 += 1}
            if correct(line, "[[6]]".as_bytes()) {acc.1 += 1}
            acc
        });
    res.0 * res.1
}

pub fn part1(){
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("Part 1 -> {}", index_sum(&input));
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("Part 2 -> {}", divider(&input));
} 
