/*
1abc2 12
pqr3stu8vwx 38
a1b2c3d4e5f 15
treb7uchet 77?
*/

fn main() {
    let input = std::fs::read_to_string("input-1.txt").unwrap();
    let sum: i32 = input
        .lines()
        .map(|line| {
            let nums: Vec<&str> = line.matches(char::is_numeric).collect();
            let num: i32 = format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
                .parse()
                .unwrap();
            num
        })
        .sum();
    println!("Sum: {}", sum);
}
