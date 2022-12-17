use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut all_shared_chs: Vec<char> = Vec::new();
    let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

    let mut line_counter: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line_counter.len() == 2 {
            line_counter.push(line);
            println!("{:?}", line_counter);
            'inner: for ch in line_counter[0].chars() {
                if line_counter[1].contains(ch) {
                    if line_counter[2].contains(ch) {
                        all_shared_chs.push(ch);
                        line_counter = Vec::new();
                        break 'inner;
                    }
                }
            }
        } else {
            line_counter.push(line)
        }
    }

    let mut char_to_point: Vec<usize> = Vec::new();
    for ch in all_shared_chs.iter() {
        char_to_point.push(alphabet.iter().position(|x| x == ch).unwrap() + 1);
    }
    println!("{:?}", char_to_point.iter().sum::<usize>())
}
