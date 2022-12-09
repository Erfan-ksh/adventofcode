use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut elf: Vec<i32> = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            elves.push(elf.clone());
            elf = Vec::new()
        } else {
            elf.push(line.parse::<i32>().unwrap())
        }
    }
    let mut sum_of_calories: Vec<i32> = Vec::new();
    for elf in elves.iter() {
        let mut sum: i32 = 0;
        for calories in elf.iter() {
            sum += calories
        }
        sum_of_calories.push(sum)
    }
    println!("{:?}", sum_of_calories.iter().max().unwrap())
}
