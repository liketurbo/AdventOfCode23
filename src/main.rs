/* Part one
1abc2 12
pqr3stu8vwx 38
a1b2c3d4e5f 15
treb7uchet 77?
*/

/* Part two
two1nine 29
eightwothree 83
abcone2threexyz 13
xtwone3four 24
4nineeightseven2 42
zoneight234 14
7pqrstsixteen 76
eighthree 83
sevenine 79
*/

const WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn replace_first_match(word: &str, words: &[String; 10]) -> String {
    let mut p_matches: Vec<_> = words
        .iter()
        .enumerate()
        .map(|(i, w)| word.find(w).and_then(|p| Some((p, (i, w)))))
        .filter(Option::is_some)
        .collect();
    p_matches.sort_by(|a, b| {
        a.as_ref()
            .expect("filtered")
            .0
            .cmp(&b.as_ref().expect("filtered").0)
    });
    if let Some(first) = { &p_matches.first() } {
        let first = first.as_ref().expect("filtered").1;
        return word.replace(first.1, &format!("{}{}", first.0, first.1));
    }

    word.into()
}

fn main() {
    let input = std::fs::read_to_string("input-1.txt").unwrap();
    let sum: i32 = input
        .lines()
        .map(|line| {
            // Replace the first digit
            let words = WORDS.map(|w| w.to_string());
            let l = replace_first_match(line, &words);
            eprintln!("Init line: {}", line);
            eprintln!("Parsed line 1/3: {}", l);
            l
        })
        .map(|line| {
            // Replace the last digit
            let rev_l = String::from(line).chars().rev().collect::<String>();
            let rev_words = WORDS.map(|w| w.chars().rev().collect::<String>());
            let l = replace_first_match(&rev_l, &rev_words)
                .chars()
                .rev()
                .collect::<String>();
            eprintln!("Parsed line 2/3: {}", l);
            l
        })
        .map(|line| {
            // Extract two-digit from the first and last digit in the string
            let nums: Vec<&str> = line.matches(char::is_numeric).collect();
            let num: i32 = format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
                .parse()
                .unwrap();
            eprintln!("Parsed line 3/3: {}", num);
            num
        })
        .sum();
    println!("Sum: {}", sum);
}
