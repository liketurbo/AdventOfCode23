use std::thread::current;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input-1.txt")?;

    let mut cards: Vec<_> = vec![];

    input.split('\n').for_each(|l| {
        if let [_card_num, rest] = l.split(':').collect::<Vec<&str>>().as_slice() {
            if let [winning, having] = rest.split('|').collect::<Vec<&str>>().as_slice() {
                cards.push(Card {
                    winning: winning
                        .trim()
                        .split(' ')
                        .filter(|c| *c != "")
                        .map(|n| n.parse().expect("no num"))
                        .collect::<Vec<_>>(),
                    current: having
                        .trim()
                        .split(' ')
                        .filter(|c| *c != "")
                        .map(|n| n.parse().expect("no num"))
                        .collect::<Vec<_>>(),
                });
            } else {
                panic!("| splitting went wrong");
            }
        } else {
            panic!(": splitting went wrong");
        }
    });

    eprintln!("parsed in num input: {:?}", cards);

    let sum: u32 = cards
        .iter()
        .map(|c| {
            let count = (c.winning.iter().filter(|n| c.current.contains(n)).count() as i32) - 1;

            if count < 0 {
                return 0;
            } else {
                (2 as u32).pow((count) as u32)
            }
        })
        .sum();

    eprintln!("sum of points of all the cards: {:?}", sum);

    Ok(())
}

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    current: Vec<u32>,
}
