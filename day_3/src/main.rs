use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut all_shared_chs: Vec<char> = Vec::new();
    let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        for ch in first.chars() {
            if second.contains(ch) {
                all_shared_chs.push(ch);
                break;
            }
        }
    }
    let mut char_to_point: Vec<usize> = Vec::new();
    for ch in all_shared_chs.iter() {
        char_to_point.push(alphabet.iter().position(|x| x == ch).unwrap() + 1);
    }
    println!("{:?}", char_to_point.iter().sum::<usize>())
}
